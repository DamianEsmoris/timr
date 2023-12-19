// GRAPH
const GRAPH_CONT = document.getElementById("clockGraph");
let lastAngle = 0;

// LIST
const TASK_CARD_CONT = document.getElementById("taskCardContainer");

const createTaskCard = ({ task, duration, start_time, end_time }) => {
    const running =  end_time === "Null";
    const cardContainer = document.createElement("section");
    cardContainer.classList.add("card");
  
    const taskSpan = document.createElement("span");
    const dotSpan = document.createElement("span");
    dotSpan.classList.add("dot");
    dotSpan.style.backgroundColor = `#${task.color}`;
    dotSpan.innerHTML = "&nbsp;";
    taskSpan.appendChild(dotSpan);
  
    const taskNameSpan = document.createElement("span");
    taskNameSpan.innerText = task.name;
    taskSpan.appendChild(taskNameSpan);
  
    cardContainer.appendChild(taskSpan);
  
    const timeSpans = document.createElement("section");
  
    const startTimeSpan = document.createElement("span");
    startTimeSpan.innerText = start_time;
    timeSpans.appendChild(startTimeSpan);
  
    const endTimeSpan = document.createElement("span");
    endTimeSpan.classList.add("sub");
    endTimeSpan.innerText = running ? "..." : end_time;
    timeSpans.appendChild(endTimeSpan);
  
    cardContainer.appendChild(timeSpans);
  
    const durationSpan = document.createElement("span");
    durationSpan.className = "tag hidden";
    durationSpan.innerText = `${duration}'`;
    cardContainer.appendChild(durationSpan);
  
    timeSpans.onmouseenter = () => {
      durationSpan.classList.remove("hidden");
      setTimeout(() => durationSpan.classList.add("visible"), 1);
    };
  
    timeSpans.onmouseleave = () => {
      durationSpan.classList.remove("visible");
      setTimeout(() => durationSpan.classList.add("hidden"), 200);
    };
  
    TASK_CARD_CONT.appendChild(cardContainer);
    return cardContainer;
  };

const createGraph = ({task, duration, end_time}) => {
    const running =  end_time === "Null";

    const graph = document.createElement('div');
    let angle = (duration * 100) / 1440;

    graph.style.background = `conic-gradient(#${task.color} 0%, #${task.color} ${angle}%, transparent ${angle}%, transparent 100%)`;
    graph.style.transform = `rotate(${lastAngle * 360 / 100}deg)`;
    GRAPH_CONT.appendChild(graph);
    lastAngle += angle;
}

const getTasks = async (date) => {
    let res = await fetch('/get_date_tasks', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json', 
        },
        body: JSON.stringify({date})
    })

    return (await res.json());
}

const createTask = (task) => {
    createTaskCard(task);
    createGraph(task);
}

const createTasks = (tasks) => {
    tasks.sort((a,b) => {
        let dA = new Date();
        let dB = new Date();

        let [hA, mA] = a.start_time.split(':');
        let [hB, mB] = b.start_time.split(':');
        
        dA.setHours(hA);
        dA.setMinutes(mA);
        
        dB.setHours(hB);
        dB.setMinutes(mB);

        return dA.getTime() - dB.getTime()
    }).forEach(t => 
        createTask(t)
    );
}

  
export {
    getTasks,
    createTasks,
    createTask
}
