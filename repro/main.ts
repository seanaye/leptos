import { serve } from "https://deno.land/std/http/server.ts"
import { serveDir } from "https://deno.land/std/http/file_server.ts";
import { instantiate } from "./lib/server.generated.js"

const { main } = await instantiate()

serve((req) => {
  const pathname = new URL(req.url).pathname
  if (pathname.startsWith("/lib/client")) {
    return serveDir(req)
  }
  return main(req)
})
