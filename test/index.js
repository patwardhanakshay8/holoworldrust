// This test file uses the tape testing framework. 
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('can create a text entry', (t) => {
  t.plan(1)
  const input = JSON.stringify({
    holo_text: "Holo World!"
  })
  const result = app.call("reader_writer", "main", "text_write", input)
  t.equal(result, JSON.stringify({ address: "QmeNBNe9JcyQssD7hXuGMUEfUGNZ4GZeJhMokM4obei1GU" }))
})

test('can list entries', (t) => {
  t.plan(1)
  const input = JSON.stringify({})
  const result = app.call("reader_writer", "main", "text_read", input)
  const parsed = JSON.parse(result)
  const expected = {
    holo_text: "Holo World!"    
  }
  t.deepEqual(parsed[0], expected)
})
