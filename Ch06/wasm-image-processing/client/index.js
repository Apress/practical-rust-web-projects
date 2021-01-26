import * as wasmImage from "wasm-image-processing"

function setup(event) {
  const fileInput = document.getElementById('image-upload')
  fileInput.addEventListener('change', function(event) {

    const file = event.target.files[0]
    const imageUrl = window.URL.createObjectURL(file)

    const image = new Image()
    image.src = imageUrl

    image.addEventListener('load', (loadEvent) => {
      const canvas = document.getElementById('preview')
      canvas.width = image.naturalWidth
      canvas.height = image.naturalHeight
      canvas.getContext('2d').drawImage(image, 0, 0)
    })
  })

  const shrinkButton = document.getElementById('shrink')
  shrinkButton.addEventListener('click', function(event) {
      const canvas = document.getElementById('preview')
      const canvasContext = canvas.getContext('2d')
      const imageBuffer = canvasContext.getImageData(0, 0, canvas.width, canvas.height).data
      const outputBuffer = wasmImage.shrink_by_half(imageBuffer, canvas.width, canvas.height)
      const u8OutputBuffer = new ImageData(new Uint8ClampedArray(outputBuffer), canvas.width / 2)

      canvasContext.clearRect(0, 0, canvas.width, canvas.height);
      canvas.width = canvas.width / 2
      canvas.height = canvas.height / 2
      canvasContext.putImageData(u8OutputBuffer, 0, 0)
  })

  const rotate90Button = document.getElementById('rotate90')
  rotate90Button.addEventListener('click', function(event) {
      const canvas = document.getElementById('preview')
      const canvasContext = canvas.getContext('2d')
      const imageBuffer = canvasContext.getImageData(0, 0, canvas.width, canvas.height).data
      const outputBuffer = wasmImage.rotate90(imageBuffer, canvas.width, canvas.height)
      const u8OutputBuffer = new ImageData(new Uint8ClampedArray(outputBuffer), canvas.height)

      canvasContext.clearRect(0, 0, canvas.width, canvas.height);
      const height = canvas.width
      const width = canvas.height
      canvas.width = width
      canvas.height = height
      canvasContext.putImageData(u8OutputBuffer, 0, 0)
  })
}

if (document.readState !== 'loading') {
  setup()
} else {
  window.addEventListener('DOMContentLoaded', setup);
}

