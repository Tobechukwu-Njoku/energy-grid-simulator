#!/usr/bin/env python3
"""Independent, streaming audit. Run with python3 -I recount.py [raw-directory].

Uses stdlib only. XLSX is copied to a temporary seekable file in bounded chunks;
XML rows and CSV records are streamed. Only shared strings, 2024 timestamp keys,
and aggregate statistics are retained. Output is derived metadata, never raw prices.
"""
import collections
import csv
from datetime import datetime, timedelta, timezone
from decimal import Decimal
import hashlib
import io
import json
from pathlib import Path
import posixpath
import re
import resource
import shutil
import sys
import tempfile
import tomllib
import xml.etree.ElementTree as ET
import zipfile
from zoneinfo import ZoneInfo

NS = '{http://schemas.openxmlformats.org/spreadsheetml/2006/main}'
UTC = timezone.utc
START = datetime(2024, 1, 1, tzinfo=UTC)
END = datetime(2025, 1, 1, tzinfo=UTC)
DUBLIN = ZoneInfo('Europe/Dublin')


def xml_items(archive, name, tag):
    with archive.open(name) as stream:
        context = ET.iterparse(stream, events=('start', 'end'))
        stack = []
        for event, element in context:
            if event == 'start':
                stack.append(element)
            else:
                if element.tag == NS + tag:
                    yield element
                    if len(stack) > 1:
                        stack[-2].remove(element)
                    element.clear()
                stack.pop()


def workbook_rows(archive, sheet_name, strings):
    rels = ET.fromstring(archive.read('xl/_rels/workbook.xml.rels'))
    targets = {r.get('Id'): r.get('Target') for r in rels}
    book = ET.fromstring(archive.read('xl/workbook.xml'))
    sheet = next(s for s in book.find(NS + 'sheets') if s.get('name') == sheet_name)
    rid = sheet.get('{http://schemas.openxmlformats.org/officeDocument/2006/relationships}id')
    target = targets[rid]
    name = target.lstrip('/') if target.startswith('/') else posixpath.normpath('xl/' + target)
    headers = None
    for row in xml_items(archive, name, 'row'):
        cells = {}
        for c in row:
            col = re.sub(r'\d', '', c.get('r'))
            value = c.findtext(NS + 'v')
            if c.get('t') == 's':
                value = strings[int(value)]
            elif c.get('t') == 'inlineStr':
                value = ''.join(c.itertext())
            cells[col] = value
        if headers is None:
            headers = cells
        else:
            yield {key: cells.get(col) for col, key in headers.items()}


class Stats:
    def __init__(self):
        self.count = 0
        self.null = 0
        self.total = Decimal(0)
        self.lo = self.hi = None

    def add(self, value):
        if value in (None, ''):
            self.null += 1
            return
        n = Decimal(value)
        if not n.is_finite():
            self.null += 1
            return
        self.count += 1
        self.total += n
        self.lo = n if self.lo is None else min(self.lo, n)
        self.hi = n if self.hi is None else max(self.hi, n)

    def result(self):
        return {'valid': self.count, 'null': self.null, 'min': str(self.lo),
                'max': str(self.hi), 'mean': str(self.total / self.count) if self.count else None}


class Series:
    def __init__(self):
        self.keys = collections.Counter()
        self.local_days = collections.Counter()
        self.stats = collections.defaultdict(Stats)
        self.flags = collections.Counter()

    def add(self, key, fields):
        self.keys[key] += 1
        self.local_days[key.astimezone(DUBLIN).date().isoformat()] += 1
        for name, value in fields.items():
            self.stats[name].add(value)

    def result(self, expected=None):
        missing = sorted(expected - self.keys.keys()) if expected is not None else []
        unexpected = sorted(self.keys.keys() - expected) if expected is not None else []
        utc_days = collections.Counter(k.date().isoformat() for k in self.keys.elements())
        return {'rows': sum(self.keys.values()), 'unique': len(self.keys),
                'duplicates': sum(n - 1 for n in self.keys.values()),
                'expected': len(expected) if expected is not None else None,
                'missing': len(missing) if expected is not None else None,
                'unexpected': len(unexpected) if expected is not None else None,
                'missing_by_month': dict(collections.Counter(t.strftime('%Y-%m') for t in missing)),
                'missing_timestamps': [t.isoformat() for t in missing],
                'first': min(self.keys).isoformat(), 'last': max(self.keys).isoformat(),
                'transition_utc_days': {d: utc_days[d] for d in ('2024-03-31','2024-10-27')},
                'transition_dublin_days': {d: self.local_days[d] for d in ('2024-03-31','2024-10-27')},
                'flags': dict(self.flags), 'stats': {k: v.result() for k,v in self.stats.items()}}


def grid(minutes, hours=None):
    t = START
    result = set()
    while t < END:
        if hours is None or hours[0] <= t.astimezone(DUBLIN).hour < hours[1]:
            result.add(t)
        t += timedelta(minutes=minutes)
    return result


def main():
    manifest = tomllib.loads(Path(__file__).with_name('manifest.toml').read_text())
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(manifest['meta']['storage_root'])
    report = {'integrity': [], 'series': {}, 'idc': {},
              'imbalance_timezone': 'ASSUMPTION: offsetless START_TIME is UTC; see Q-C-11',
              'five_minute_Y_with_missing_volume': 0,
              'five_minute_other_flags_with_missing_volume': 0}
    series = collections.defaultdict(Series)
    delivery_dam = Stats()
    for entry in manifest['datasets']:
        path = root / entry['relative_path']
        digest = hashlib.sha256()
        with path.open('rb') as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b''):
                digest.update(chunk)
        measured = {'filename': path.name, 'bytes': path.stat().st_size, 'sha256': digest.hexdigest()}
        assert measured['bytes'] == entry['bytes'] and measured['sha256'] == entry['sha256'], measured
        report['integrity'].append(measured)
        with zipfile.ZipFile(path) as outer:
            if entry['category'] == 'semopx':
                with tempfile.TemporaryFile() as tmp:
                    with outer.open(next(n for n in outer.namelist() if n.endswith('.xlsx'))) as stream:
                        shutil.copyfileobj(stream, tmp, 1024 * 1024)
                    tmp.seek(0)
                    with zipfile.ZipFile(tmp) as book:
                        strings = [''.join(si.itertext()) for si in xml_items(book, 'xl/sharedStrings.xml', 'si')]
                        for row in workbook_rows(book, 'auctions_to', strings):
                            if row['auction'] == 'DAM' and row.get('year') == '2024':
                                delivery_dam.add(row['price_eur'])
                            if not (row['timestamp'] or '').startswith('2024-'):
                                continue
                            stamp = datetime.fromisoformat(row['timestamp'])
                            assert stamp.tzinfo is not None, row['timestamp']
                            if START <= stamp < END:
                                series[row['auction']].add(stamp, {k: row[k] for k in ('price_eur','price_gbp')})
                        keys = collections.Counter()
                        exact_rows = collections.Counter()
                        idcstats = Stats()
                        for row in workbook_rows(book, 'idcs_to', strings):
                            if not row['deliverydate']:
                                continue
                            day = (datetime.strptime(row['deliverydate'], '%d/%m/%Y') if '/' in row['deliverydate']
                                   else datetime(1899,12,30) + timedelta(days=float(row['deliverydate'])))
                            if day.year == 2024:
                                keys[(day.date().isoformat(), row['deliveryhour'])] += 1
                                exact_rows[tuple(row.values())] += 1
                                idcstats.add(row['idc_avg_price'])
                        report['idc'] = {'rows': sum(keys.values()), 'identical_duplicate_rows': sum(n-1 for n in exact_rows.values()), 'unique_date_hour_keys': len(keys),
                            'repeated_date_hour_keys': sum(n > 1 for n in keys.values()),
                            'excess_rows_on_date_hour_key': sum(n-1 for n in keys.values()),
                            'maximum_rows_per_date_hour': max(keys.values()),
                            'transition_rows': {d: sum(n for (day,h),n in keys.items() if day==d)
                                                for d in ('2024-03-31','2024-10-27')},
                            'stats': idcstats.result()}
            else:
                with outer.open(next(n for n in outer.namelist() if n.endswith('.csv'))) as stream:
                    for row in csv.DictReader(io.TextIOWrapper(stream, encoding='utf-8-sig')):
                        stamp = datetime.strptime(row['START_TIME'], '%d/%m/%Y %H:%M').replace(tzinfo=UTC)
                        if START <= stamp < END:
                            s = series[row['SUMM_TYPE']]
                            s.add(stamp, {k: row[k] for k in ('FINAL_IMBALANCE_PRICE','NET_IMBALANCE_VOLUME')})
                            s.flags[row['PRICE_FLAG']] += 1
                            if row['SUMM_TYPE'] == '5MIN' and row['NET_IMBALANCE_VOLUME'] == '':
                                field = ('five_minute_Y_with_missing_volume' if row['PRICE_FLAG'] == 'Y'
                                         else 'five_minute_other_flags_with_missing_volume')
                                report[field] += 1
    for name,s in series.items():
        expected = grid(60 if name == 'DAM' else 5 if name == '5MIN' else 30,
                        (11,23) if name == 'IDA2' else (17,23) if name == 'IDA3' else None)
        report['series'][name] = s.result(expected)
    report['dam_workbook_delivery_year_2024'] = delivery_dam.result()
    report['peak_rss_kib_linux'] = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    json.dump(report, sys.stdout, indent=2)
    print()


if __name__ == '__main__':
    main()
