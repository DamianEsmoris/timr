const CARD_CONT = document.getElementById('cardsContainer');
const NAME_DIALOG = Object.freeze({
    self: document.getElementById('nameDialog'),
    input: document.getElementById('dialogNameInput'),
    name: {
        old: document.getElementById('oldName'),
        new: document.getElementById('newName'),
    },
    btns: {
        ok: document.getElementById('nameDialogOk'),
        cancel: document.getElementById('nameDialogCancel'),
    }
});

const DELETION_DIALOG = Object.freeze({
    self: document.getElementById('deletionDialog'),
    name: document.getElementById('deletionTaskName'),
    btns: {
        ok: document.getElementById('deletionDialogOk'),
        cancel: document.getElementById('deletionDialogCancel'),
    }
});
const MENU = Object.freeze({
    cont: document.getElementById('contextualMenuCont'),
    self: document.getElementById('contextualMenu'),
    name: document.getElementById('menuTaskName'),
    btns: {
        update: document.getElementById('menuUpdateBtn'),
        start: document.getElementById('menuStartBtn'),
        edit: {
            name: document.getElementById('editSubmenuChangeName'),
            delete: document.getElementById('editSubmenuDelete'),
        }
    }
})


let contextMenuOpen = false;

const createCard = ({name, color}) => {
    let container = document.createElement('section');

    let span = document.createElement('span');
    span.textContent = name;
    container.appendChild(span);

    let colorSpan = document.createElement('span');
    colorSpan.textContent = `#${color}`;
    container.appendChild(colorSpan);

    let colorPicker = document.createElement('input');
    colorPicker.type = 'color';
    colorPicker.defaultValue = `#${color}`;
    container.appendChild(colorPicker);

    colorPicker.onchange = () => {
        colorSpan.textContent = colorPicker.value;
    }

    CARD_CONT.appendChild(container);

    container.oncontextmenu = (e) => {
        e.preventDefault();
        if(contextMenuOpen){
            MENU.cont.classList.remove('active');
            setTimeout(() => openMenu(e, {name, color: `#${color}`}, {colorPicker}), 310);
        }else openMenu(e, {name, color: `#${color}`}, {colorPicker} )
    }
}

const openMenu = (e, task, obj) => {
    closeMenu();
    updateMenu(task, obj);
    MENU.cont.style.top = `${e.pageY}px`;
    MENU.cont.style.left = `${e.pageX}px`;

    MENU.cont.classList.add('active');

    contextMenuOpen = true;
}

const updateMenu = (task, obj) => {
    MENU.name.textContent = task.name;
    MENU.btns.update.onclick = () => {
        if(obj.colorPicker.value != task.color){
            alert('cambiaste el color')
            console.log(task.color);
            task.color = obj.colorPicker.value;
            console.log(task.color);
        }
    }
    
    MENU.btns.edit.name.onclick = () => {
        NAME_DIALOG.self.showModal();

        NAME_DIALOG.name.old.textContent = task.name;
        NAME_DIALOG.input.onkeyup = () => {
            let newValue = NAME_DIALOG.input.value;
            NAME_DIALOG.name.new.textContent = newValue.length ? newValue : "please type";
        }
    }

    
    MENU.btns.edit.delete.onclick = () => {
        DELETION_DIALOG.self.showModal();
        DELETION_DIALOG.name.innerHTML = `Do you want to delete the tasks <i>'${task.name}'</i>`;
    }
}

const closeMenu = () => MENU.cont.classList.remove('active');

const getTasks = async () => {
    let res = await fetch('/get_tasks', {
        method: 'post'
    });
    return (await res.json());
}

getTasks().then(tasks => 
    tasks.forEach(t => createCard(t))
)

document.onclick = (e) => {
    if(!MENU.cont.contains(e.target))
        closeMenu();
}


///
let startX, scrollLeft, isScrolling = false;
CARD_CONT.onmousedown = (e) => {
    isScrolling = true;
    startX = e.pageX - CARD_CONT.offsetLeft;
    scrollLeft = CARD_CONT.scrollLeft
}



CARD_CONT.onmousemove = (e) => {
    if(!isScrolling) return false; 
    e.preventDefault();
    let x = e.pageX - CARD_CONT.offsetLeft;
    let walk = x - startX;
    CARD_CONT.scrollLeft = scrollLeft - walk;
}

CARD_CONT.onmouseup = () => isScrolling = false;
CARD_CONT.onmouseleave = () => isScrolling = false;