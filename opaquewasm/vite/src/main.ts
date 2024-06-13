import { setupRegister } from './register.ts'


document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
    <h1>Vite + TypeScript + WASM</h1>
    <div>
      <button id="button" type="button"></button>
    </div>
  </div>
`

setupRegister(document.querySelector<HTMLButtonElement>('#button')!)
