import { useSocket } from '../statusManager.js';
import {CLOCK, createDialLine, updateClock} from './clock.js';
import { createTasks, getTasks } from './taskManager.js';

let d = new Date();
getTasks(d.toLocaleDateString("en-CA")).then(tasks => {
  createTasks(tasks);
})

createDialLine();
setInterval(updateClock, 80);
document.body.onresize = () => {
  Array.from(CLOCK.querySelectorAll(".dialline")).forEach(
      (d) => (d.style.transformOrigin = `50% ${CLOCK.clientWidth / 2}px`)
  );
};


let statusElement;
const createStatusCont = () => {
	const cont = document.createElement("section");

	const textCont = document.createElement("section");
	const title = document.createElement("span");
	const stop = document.createElement("span");
	const time = document.createElement("span");

	cont.id = "bottomTaskContainer";
	stop.className = "btn nerd-icon";
	time.className = "sub";

	stop.textContent = "󰙧";
	time.textContent = "19 mins";

	textCont.appendChild(title);
	textCont.appendChild(stop);
	cont.appendChild(textCont);
	cont.appendChild(time);
	document.body.appendChild(cont);
	
	return {
		cont,	
		title,
		time,
		stop
	}
}

const hideBottomCard= () => {
	if(statusElement) 
		statusElement.cont.classList.remove("active");
}	

const updateBottomCard = (t) => {
	if(!statusElement) 
		statusElement = createStatusCont();

	statusElement.cont.classList.add("active");
	statusElement.title.textContent = t.task.name;
	statusElement.time.textContent = `${t.duration} mins`;
	statusElement.stop.onclick = () => {
		fetch("/stop_task", {
			headers: {
            'Content-Type': 'application/json',
      },			
			method: "post",
			body: JSON.stringify({name: t.task.name})
		})
		.then(res => {
			if (!res.ok) return false;
			hideBottomCard();
			fetch("/stop_task_notify");
		);

	}
}

const socketCallback = (data) => {
	console.log(data);
	switch (data.action) {
		case "task":
			updateBottomCard(data.content);
		break;

		case "start_task":
		break;
	} 
}


useSocket(socketCallback);

