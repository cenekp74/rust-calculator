const { invoke } = window.__TAURI__.tauri

const allowedCharactersRegex = /[0123456789/*\-+()^!]/;
const in_ele = document.getElementById('input');
const err_ele = document.getElementsByClassName('error')[0];

async function process() {
    await invoke('process', { input: in_ele.value })
        .then((response) => {
            if (response.includes('error')) {
                err_ele.innerHTML = response
                return
            };
            err_ele.innerHTML = ''
            in_ele.value = response
    })
}

async function test() {
    await invoke('test')
        .then((response) => {
            console.log(response)
    })
}

function handleButtonClick(ele) {
    action = ele.id;
    if (action == 'eq') return;
    if (action == 'del') {
        in_ele.value = in_ele.value.slice(0, -1);
        return
    }
    if (action == 'ac') {
        err_ele.innerHTML = '';
        in_ele.value = '';
        return
    }
    in_ele.value += action;
}

function handleKeyPress(e) {
    source = e.target;
    if (source.tagName.toLowerCase() == 'input') return;

    key = e.key;
    
    if (key == 'Backspace') {
        in_ele.value = in_ele.value.slice(0, -1);
        return
    }
    if (key == '=') process();
    if (key == 'Enter') process();

    if (!allowedCharactersRegex.test(key)) return;
    in_ele.value += key;
}

buttons = document.querySelectorAll(".calculator-button");
buttons.forEach(function (element) {
    element.addEventListener('click', function () {
        handleButtonClick(element);
    });
});

in_ele.addEventListener('keydown', function(event) {
    if (event.key == 'Enter') process();
    if(event.ctrlKey
    || event.altKey
    || typeof event.key !== 'string'
    || event.key.length !== 1) {
        return;
    }
    
    if(!allowedCharactersRegex.test(event.key)) {
        event.preventDefault();
    }
}, false);

document.addEventListener('keydown', handleKeyPress);