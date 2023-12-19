const CLOCK = document.getElementById("clock");
const DIALLINES_CONT = document.getElementById("dialLinesContainer");
let firstRefresh = true;

const createDialLine = () => {
  let amount = 60;
  let spaceBetween = 6;

  for (let i = 1; i <= amount; i += amount / spaceBetween / 10) {
    let diaLine = document.createElement("span");
    diaLine.style.transform = `rotate(${spaceBetween * i}deg)`;
    diaLine.style.transformOrigin = `50% ${CLOCK.clientWidth / 2}px`;
    diaLine.className = "dialline";
    DIALLINES_CONT.appendChild(diaLine);
  }
};

const NEEDLES = Array.from(CLOCK.querySelectorAll(".needle"));
const TIME_UNITS = Object.freeze({
  seconds: {
    max: 60,
    obj: NEEDLES[0],
    lastValue: null
  },

  minutes: {
    max: 60,
    obj: NEEDLES[1],
    lastValue: null
  },

  hours: {
    max: 12*60,
    obj: NEEDLES[2],
    lastValue: null
  },
})

const rotateNeedle = (time, timeUnit) => {
  if (timeUnit.lastValue === time)
    return false;

timeUnit.lastValue = time;
  if(time === 0)
    time = timeUnit.max;

  let rotation = (time * 360) / timeUnit.max;
  rotation += 270;

  timeUnit.obj.style.transform = `rotate(${rotation}deg)`;

  if(time === timeUnit.max)
  setTimeout(() => {
    timeUnit.obj.classList.add('transition-disabled');
    timeUnit.obj.style.transform = 'rotate(270deg)';
    setTimeout(() => timeUnit.obj.classList.remove('transition-disabled'), 10);
  }, 350);
};


Date.prototype.getHours12Format = function() {
  let hours = this.getHours();
  hours = hours % 12;
  return hours ? hours : 12; 
}

const updateClock = () => {
    let d = new Date();
    rotateNeedle(d.getSeconds(), TIME_UNITS.seconds);
    rotateNeedle(d.getMinutes(), TIME_UNITS.minutes);
    rotateNeedle(d.getHours12Format() * 60 + d.getMinutes(), TIME_UNITS.hours);
    if(firstRefresh){
      setTimeout(() => 
        NEEDLES.forEach(n => n.classList.remove('transition-disabled'))
      , 10);
    }
}

export {
    CLOCK,
    createDialLine,
    updateClock
};