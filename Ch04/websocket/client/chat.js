document.addEventListener("DOMContentLoaded", function(){
  const socket = new WebSocket("ws://127.0.0.1:8080");
  socket.onmessage = function (event) {
    const messages = document.getElementById("messages");
    messages.value += `${event.data}\n`;
  };

  const sendButton= document.getElementById("send");
  sendButton.addEventListener("click", (event) => {
    const message = document.getElementById("message");
    socket.send(message.value)
    message.value = "";
  })
});
