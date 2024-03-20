import * as streams from "https://deno.land/std@0.177.1/streams/mod.ts";
import { parse } from "https://deno.land/std@0.177.1/flags/mod.ts";

const HELP = `
Usage: reload <--file> [--port=31337]

Options:
  -h, --help
    Print help
  -f, --file
    File to sync with ImHex
  -p, --port
    Port to connect to ImHex with
`.trim()

const args = parse(Deno.args, {
  alias: {
    'p': 'port',
    'f': 'file',
    'h': 'help'
  },
  string: [
    'file'
  ],
  boolean: [
    'help'
  ],
  default: {
    'port': 31337
  }
})

if (args.help) {
  console.log(HELP)
  Deno.exit(0)
}

const fileArg = args.file ?? args._.shift()?.toString()
const portArg = args._.shift() ?? args.port

if (fileArg == null) {
  console.error(HELP)
  Deno.exit(1)
}

if (typeof portArg !== 'number') {
  throw new Error(`--port must be number, not: ${typeof portArg}`)
}

const encoder = new TextEncoder()
const decoder = new TextDecoder()
const encodeJSON = (value: unknown) => encoder.encode(JSON.stringify(value))

async function sendCommand(endpoint: string, data: unknown) {
  const connection = await Deno.connect({ port: portArg as number, transport: 'tcp' });
  connection.setKeepAlive(false)
  connection.setNoDelay(true)

  await streams.writeAll(connection, encodeJSON({ endpoint, data }));
  
  const response = await streams.readAll(connection)
    .then((res) => decoder.decode(res))
    .then((res) => JSON.parse(res));

  connection.close()

  return response
}

async function syncCode(withFile: string) {
  const code = await Deno.readTextFile(withFile)
  return sendCommand('pattern_editor/set_code', { code })
}

console.log('trying to sync...')
console.log(await syncCode(fileArg))
const watcher = Deno.watchFs(fileArg);
for await (const event of watcher) {
  if (event.kind !== 'modify') continue
  console.log(Date.now(), 'attempting to sync code')
  console.log(await syncCode(fileArg))
}