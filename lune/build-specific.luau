local fs = require("@lune/fs")
local process = require("@lune/process")
local stdio = require("@lune/stdio")

local crates = fs.readDir("crates")
table.sort(crates)

local crate_selected = crates[stdio.prompt("select", "select crate to build", crates)]

local result = process.spawn("cargo", { "v5", "build" }, {
	cwd = "crates/" .. crate_selected,
})

print(result.stdout)
print(result.stderr)
