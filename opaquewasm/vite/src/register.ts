import { client_register_wasm } from 'opaquewasm';
//import { client_register_wasm } from '@tiptenbrink/opaquewasm';

function regStart(password: string) {
  const { message, state } = client_register_wasm(password)

    console.log(state)
  

  // Pj8bFY58CZoyi9Rsp2KyS4HhA2vXcSEAFH7BViwxRzw
  // Pj8bFY58CZoyi9Rsp2KyS4HhA2vXcSEAFH7BViwxRzwAhBEcXSqitQsZKc2lmpI0vv4o_nocam_Lcc5QLGZ2AWdhcmJhZ2U

  return message
}

export function setupRegister(element: HTMLButtonElement) {
  const setValue = () => {
    element.innerHTML = `message is ${regStart('abc')}`
  }
  element.addEventListener('click', () => setValue())
  setValue()
}
