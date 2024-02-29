const { invoke } = window.__TAURI__.tauri

async function process() {
    in_ele = document.getElementById('input')
    await invoke('process', { input: in_ele.value })
        .then((response) => {
            window.result.innerHTML = response
    })
}

async function test() {
    await invoke('test')
        .then((response) => {
            window.result.innerHTML = response
    })
}