import collections
import csv
import json
from pathlib import Path
import re
import zipfile

import openpyxl


ROOT = Path(__file__).resolve().parent
RAW = ROOT / "raw"


def connectivity(nodes, edges):
    adjacency = {node: set() for node in nodes}
    for first, second in edges:
        adjacency.setdefault(first, set()).add(second)
        adjacency.setdefault(second, set()).add(first)
    unseen = set(adjacency)
    sizes = []
    while unseen:
        pending = [unseen.pop()]
        size = 0
        while pending:
            current = pending.pop()
            size += 1
            for neighbour in adjacency[current] & unseen:
                unseen.remove(neighbour)
                pending.append(neighbour)
        sizes.append(size)
    return {"components": len(sizes), "largest_sizes": sorted(sizes, reverse=True)[:20],
            "isolated_nodes": sum(size == 1 for size in sizes)}


book = openpyxl.load_workbook(RAW / "etys-2025-appendix-b.xlsx", read_only=True, data_only=True)
etys = {"sheets": {}, "index_voltage_rows": {}}
edges = []
parameter_gaps = collections.Counter()
for sheet in book:
    if sheet.title.startswith("B-1-1"):
        rows = [row for row in list(sheet.values)[2:] if row[0] and isinstance(row[2], (int, float))]
        etys["index_voltage_rows"][sheet.title] = dict(collections.Counter(str(row[2]) for row in rows))
    if not sheet.title.startswith(("B-2-1", "B-3-1")):
        continue
    values = list(sheet.values)
    headers = [re.sub(r"\s", "", str(value)).lower() for value in values[1]]
    first_index = headers.index("node1")
    second_index = headers.index("node2")
    parameter_indices = [headers.index(header) for header in headers if header.startswith(("r(%", "x(%", "b(%"))]
    rating_indices = [index for index, header in enumerate(headers) if "rating" in header]
    rows = [row for row in values[2:] if row[first_index] and row[second_index]]
    etys["sheets"][sheet.title] = len(rows)
    for row in rows:
        edges.append((str(row[first_index]).strip(), str(row[second_index]).strip()))
        for index in parameter_indices + rating_indices:
            if not isinstance(row[index], (int, float)):
                parameter_gaps[headers[index]] += 1
nodes = {node for edge in edges for node in edge}
etys["endpoint_nodes"] = len(nodes)
etys["branch_rows"] = len(edges)
etys["exact_endpoint_graph"] = connectivity(nodes, edges)
site_edges = [(first[:4], second[:4]) for first, second in edges]
etys["site_collapsed_diagnostic"] = {"sites": len({node[:4] for node in nodes}),
                                        **connectivity({node[:4] for node in nodes}, site_edges)}
etys["nonnumeric_electrical_or_rating_cells"] = dict(parameter_gaps)
etys["repeated_unordered_endpoint_pairs"] = sum(count - 1 for count in collections.Counter(tuple(sorted(edge)) for edge in edges).values())

with zipfile.ZipFile(RAW / "tytfs-2024.zip") as archive:
    member = "TYTFS2024_studyfiles/TYTFS2024_WP2024_V33.raw"
    lines = archive.read(member).decode("utf-8-sig").splitlines()
sections = collections.defaultdict(list)
section = "BUS"
for line in lines[3:]:
    if line.lstrip().startswith("0 / END OF"):
        match = re.search(r"BEGIN (.*?) DATA", line)
        section = match.group(1) if match else "END"
    elif line.strip() and line.strip() != "Q":
        sections[section].append(next(csv.reader([line.split(" / ")[0]], quotechar="'", skipinitialspace=True)))
buses = {int(row[0]): row for row in sections["BUS"]}
branches = sections["BRANCH"]
transformers = []
cursor = 0
while cursor < len(sections["TRANSFORMER"]):
    record = sections["TRANSFORMER"][cursor]
    transformers.append(record)
    cursor += 5 if int(record[2]) != 0 else 4
assert cursor == len(sections["TRANSFORMER"])
all_edges = [(int(row[0]), int(row[1])) for row in branches]
active_edges = [(int(row[0]), int(row[1])) for row in branches if int(row[13]) == 1]
for record in transformers:
    terminals = [int(value) for value in record[:3] if int(value) != 0]
    terminal_connectivity_edges = [(terminals[0], other) for other in terminals[1:]]
    all_edges.extend(terminal_connectivity_edges)
    status = int(record[11])
    if status == 1:
        active_edges.extend(terminal_connectivity_edges)
    elif status in {2, 3, 4} and len(terminals) == 3:
        disconnected_winding = {2: 1, 3: 2, 4: 0}[status]
        live_terminals = [node for index, node in enumerate(terminals) if index != disconnected_winding]
        active_edges.append(tuple(live_terminals))
active_buses = {number for number, row in buses.items() if int(row[3]) != 4}
missing = {node for edge in all_edges for node in edge} - buses.keys()
target_buses = {number for number, row in buses.items() if float(row[2]) in {400, 380, 275, 220, 110}}
active_target_buses = target_buses & active_buses
ireland = {"case": member, "bus_records": len(buses),
           "transformer_graph_representation": "physical-terminal connectivity projection; not an electrical equivalent",
           "bus_voltage_counts": dict(collections.Counter(row[2].strip() for row in buses.values())),
           "bus_type_counts": dict(collections.Counter(row[3].strip() for row in buses.values())),
           "ac_branch_records": len(branches), "in_service_ac_branches": sum(int(row[13]) == 1 for row in branches),
           "transformer_devices": len(transformers),
           "three_winding_devices": sum(int(row[2]) != 0 for row in transformers),
           "in_service_transformers_status_1": sum(int(row[11]) == 1 for row in transformers),
           "other_transformer_status_counts": dict(collections.Counter(row[11].strip() for row in transformers if int(row[11]) != 1)),
           "generator_records": len(sections["GENERATOR"]),
           "missing_branch_or_transformer_bus_references": len(missing),
           "all_records_graph": connectivity(set(buses), all_edges),
           "in_service_graph": connectivity(active_buses, [(first, second) for first, second in active_edges if first in active_buses and second in active_buses]),
           "target_voltage_buses_including_380_base": len(target_buses),
           "active_target_voltage_buses": len(active_target_buses),
           "target_voltage_induced_graph": connectivity(active_target_buses, [(first, second) for first, second in active_edges if first in active_target_buses and second in active_target_buses]),
           "ac_branch_nonnumeric_r_x_b_rating_a": sum(any(not re.fullmatch(r"[-+0-9.eE]+", row[index].strip()) for index in [3, 4, 5, 6]) for row in branches),
           "ac_branch_zero_rating_a": sum(float(row[6]) == 0 for row in branches)}
result = {"etys_2025_26": etys, "ireland_winter_peak_2024": ireland}
output = ROOT / "samples" / "network-statistics.json"
output.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
