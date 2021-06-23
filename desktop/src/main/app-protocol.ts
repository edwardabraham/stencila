/*
Based on Reasonably Secure Electron
@see https://github.com/reZach/secure-electron-template/blob/master/app/electron/protocol.js

Copyright (C) 2021  Bishop Fox
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
-------------------------------------------------------------------------

Implementing a custom protocol achieves two goals:
  1) Allows us to use ES6 modules/targets
  2) Avoids running the app in a file:// origin
*/

import { Protocol } from 'electron'
import * as fs from 'fs'
import * as path from 'path'

export const scheme = 'stencila'
const DIST_PATH = path.join(__dirname, '../renderer/')

const mimeTypes = {
  '.js': 'text/javascript',
  '.mjs': 'text/javascript',
  '.html': 'text/html',
  '.htm': 'text/html',
  '.json': 'application/json',
  '.css': 'text/css',
  '.ico': 'image/vnd.microsoft.icon',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.map': 'text/plain',
}

function mime(filename: string): string {
  // @ts-ignore
  const type = mimeTypes[path.extname(`${filename || ''}`).toLowerCase()]
  return type ? type : null
}

type RequestHandler = Parameters<Protocol['registerBufferProtocol']>['1']

export const requestHandler: RequestHandler = (req, next) => {
  const reqUrl = new URL(req.url)

  // If the path doesn't start with "/" then path.normalize will not
  // resolve all '..' and could lead to path traversal attacks.
  if (!reqUrl.pathname.startsWith('/')) {
    return next({
      mimeType: undefined,
      charset: undefined,
      data: undefined,
    })
  }

  let reqPath = path.normalize(reqUrl.pathname)

  // This is necessary to avoid trying to resolve dynamic routes as file on the filesystem
  // TODO: Investigate a more refined way of detecting if we’re loading an asset or a route within the app.
  if (reqPath === '/' || !reqPath.includes('.')) {
    reqPath = '/main_window/index.html'
  }

  const reqFileName = path.basename(reqPath)
  const filePath = path.join(DIST_PATH, reqPath)

  fs.readFile(filePath, (err, data) => {
    const mimeType = mime(reqFileName)
    if (!err && mimeType !== null) {
      next({
        mimeType: mimeType,
        charset: 'utf-8',
        data: data,
      })
    } else {
      console.error(err)
    }
  })
}
