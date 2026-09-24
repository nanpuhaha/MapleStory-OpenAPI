const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null

try {
  nativeBinding = require('./maplestory-openapi.node')
} catch (e) {
  try {
    nativeBinding = require(`./maplestory-openapi.${platform}-${arch}-msvc.node`)
  } catch (e2) {
    try {
      nativeBinding = require(`@maplestory-openapi/binding-${platform}-${arch}`)
    } catch (e3) {
      try {
        nativeBinding = require('./index.node')
      } catch (e4) {
        throw new Error(`Failed to load native binding: ${e4}`)
      }
    }
  }
}

module.exports = nativeBinding
