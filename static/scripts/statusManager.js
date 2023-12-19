
const useSocket = (msgCallback) => {
    const socket = new WebSocket("ws://192.168.1.40:8080/ws/");
        
    socket.addEventListener('message', (event) => {
        const data = JSON.parse(event.data);
        data.content = JSON.parse(data.content);
				msgCallback(data);
    });
}

export {
    useSocket
}
