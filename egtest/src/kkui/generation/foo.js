const fs = require('fs')
const path = require('path')

const input = path.resolve(__dirname, './font2.c')
const out = path.resolve(__dirname, './font3.rs')

const foo = fs.readFileSync(input, { encoding: 'utf-8' }).split('\n')
if (fs.existsSync(out)) fs.unlinkSync(out)
fs.appendFileSync(out, `#![allow(dead_code)]\n\nuse embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};\n`)

while (foo.length > 0) {
  const line1 = foo.shift()
  const line2 = foo.shift()
  const line3 = foo.shift()
  if (line3 !== '') throw new Error(`wat1`)
  const bits = /^static const CharacterImage (\w+) = \{\w+, ?(\d+), ?(\d+)\};$/.exec(line2)
  if (!bits) throw new Error(`wat2: ${line2}`)
  const peices = /^static const uint8_t image_(?:data_|font_)?(\w+)\[([^\]]+)\] = \{([^}]+)\};$/.exec(line1)
  if (!peices) throw new Error(`wat3: ${line1}`)
  if (bits[1] !== peices[1]) throw new Error(`wat4: ${bits[1]} !== ${peices[1]}`)
  const rowlen = parseInt(bits[2])
  const height = parseInt(bits[3])
  if (!((height == 10 && !peices[1].startsWith("pin")) || (height == 12 && peices[1].startsWith("pin")))) throw new Error(`wat999: ${height}`)
  if (parseInt(bits[2]) * parseInt(bits[3]) !== parseInt(peices[2])) throw new Error(`wat5(${bits[1]}): ${parseInt(bits[2])} * ${parseInt(bits[3])} !== ${parseInt(peices[2])}`)
  fs.appendFileSync(out, `\npub const ${peices[1].toUpperCase()}: ImageRaw::<BinaryColor> = ImageRaw::<BinaryColor>::new_binary(&[\n    `)
  let baz = []
  let bar = Object.create(baz)
  bar.push = function(x) {
    baz.push(x)
    bar.flush()
  }
  bar.flush = (force = false) => {
    if (bar.length === 0 && bar.bc === 0) return
    if (bar.length >= 0) {
      if (bar.length < 8 && !force) return
      fs.appendFileSync(out, `0b${bar.join('')}${bar.length < 8 ? '_' : ''}`)
      for (let i = 0; i < (8 - bar.length); i++) {
        fs.appendFileSync(out, '0')
      }
      bar.bc++;
    }
    if (force) {
      fs.appendFileSync(out, ',\n    ')
      bar.bc = 0;
    } else {
      fs.appendFileSync(out, ', ')
    }
    while (baz.length > 0) baz.pop()
  }
  bar.bc = 0
  for (let foo of peices[3].split(',')) {
    foo = foo.trim()
    switch (foo) {
      case '0x00':
        bar.push('1')
        break
        case '0xff':
        bar.push('0')
        break
      default:
        throw new Error(`wat5: ${foo} ${peices[3]}`)
    }
    if (bar.length + bar.bc * 8 >= rowlen) bar.flush(true)
  }
  bar.flush(true)
  fs.appendFileSync(out, `], ${rowlen});\n`)
}

fs.writeFileSync(out, fs.readFileSync(out, { encoding: 'utf8' }).replaceAll(/^    \]/mg, ']').replaceAll(/ (0b_00000000|0b_11111111),$/mg, ''))
