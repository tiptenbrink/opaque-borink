import * as opaque from "opaquewasm";

const port = 4243

function regStart(password) {
    const { message, state } = opaque.client_register_wasm(password)
    console.log(message)
    console.log(state)

    // Pj8bFY58CZoyi9Rsp2KyS4HhA2vXcSEAFH7BViwxRzw
    // Pj8bFY58CZoyi9Rsp2KyS4HhA2vXcSEAFH7BViwxRzwAhBEcXSqitQsZKc2lmpI0vv4o_nocam_Lcc5QLGZ2AWdhcmJhZ2U
}


function regFinish() {
    // password 'garbage'
    const in_message = "ImY8xif6bPnl6VxtZhoQcLj0vlvIm6ScFskQRkIRU1Q6EpuPqvN2i31z3L9EJwVwgpijMXY5o4dbsDr5SG95Cg"
    const state = "Pj8bFY58CZoyi9Rsp2KyS4HhA2vXcSEAFH7BViwxRzwAhBEcXSqitQsZKc2lmpI0vv4o_nocam_Lcc5QLGZ2AWdhcmJhZ2U"
    const message = opaque.client_register_finish_wasm(state, in_message)
    console.log(message)
}

function loginStart(password) {
    const { message, state } = opaque.client_login_wasm(password)
    console.log(message)
    console.log(state)

    // UD06GXLMCcJr-EaYonw0zKGQ9FMeMJ55Mh_H5yJ2S1AuF_sQmykFADMj9vdgA1Umw2SwtH0Tai0lOdF1WAM0TAAA_gVx9nSVv9YgIw5aMsrg67LJTZBm7DDQG4O6XpK9Rlw
    // uqj03DWnNUad_LIbpTDIJaUrk8zBLapBeVIml8DGWQcAYlA9OhlyzAnCa_hGmKJ8NMyhkPRTHjCeeTIfx-cidktQLhf7EJspBQAzI_b3YANVJsNksLR9E2otJTnRdVgDNEwAAP4FcfZ0lb_WICMOWjLK4OuyyU2QZuww0BuDul6SvUZcAEDhOYz3EXbCUN0CHiclrbapBa-zfhCd0A0HYliXyMNrCS4X-xCbKQUAMyP292ADVSbDZLC0fRNqLSU50XVYAzRMZ2FyYmFnZQ
}

function loginFinish() {
    const in_message = "0NOpl7MrufzQOZHO2aZ06grelr5aKNh4exSuNI5zJG46EpuPqvN2i31z3L9EJwVwgpijMXY5o4dbsDr5SG95CgGYMBvFVxtSnkxUWbvn10LPli81iWlnDk1ChpAUAkeQ-a58rf8UV1XzudDtt-nsFnkFRFmAuno79vQWUTzQgqSxNAVc3aajxCqtWUhA8nUOIC2vkUa5eppKR25iQnIZEtfL9HNKaQwTwWzXNipM8z58DPNGNtEkF84p1y6IlLPIVXQkGaG_mkfCyAkWCOfs-JRq_SqaDe1VpwYm1N7-Kmv75AGFSSX668teAEBjq9yPVUuNUVOw4MKIpUKq9wovKFkAAATMILO7ygCb0wPPe6qN8j1LJ2ZuSvyasjJbPeIZ2ECkSedAf9OBas_HuS6-H4ABjaCW2ZC5YQJPCfOsp1c2Im8"
    const state = "uqj03DWnNUad_LIbpTDIJaUrk8zBLapBeVIml8DGWQcAYlA9OhlyzAnCa_hGmKJ8NMyhkPRTHjCeeTIfx-cidktQLhf7EJspBQAzI_b3YANVJsNksLR9E2otJTnRdVgDNEwAAP4FcfZ0lb_WICMOWjLK4OuyyU2QZuww0BuDul6SvUZcAEDhOYz3EXbCUN0CHiclrbapBa-zfhCd0A0HYliXyMNrCS4X-xCbKQUAMyP292ADVSbDZLC0fRNqLSU50XVYAzRMZ2FyYmFnZQ"
    const { message, session } = opaque.client_login_finish_wasm(state, in_message)
    console.log(message)
    console.log(session)
}


async function register(username, password) {
    try {
        const { message: message1, state } = opaque.client_register_wasm(password)

        console.log(message1)
        console.log(state)

        // get message to server and get message back
        const reqst = {
            "username": username,
            "client_request": message1
        }
        const res = await fetch(`http://localhost:${port}/auth/register/start`, {
            method: 'POST', body: JSON.stringify(reqst),
            headers: {
                'Content-Type': 'application/json'
            }
        })
        const parsed = await res.json()
        const server_message = parsed.server_message
        const auth_id = parsed.auth_id
        const register_state = state
        console.log(auth_id)
        console.log(server_message)

        // pass 'abc'
        //const login_state = "Gg6GSd_2X9ccTkVZBatUyynmRM5CWBVh9j8Fsac2hQAAYoxXlNs3YTKM_4eq-Tr3hOM5TO1OZTaAgI7DYQIV4rhX-EomurCCwcw3cojfbBudPS6aF0YyxJZYbjgD8ABTigIAAMaJ77uRiMGm50uF6_VEFchFlKmwvKhhiUUsRhZhRl1fAEChX0fsJTWoEsS2bPTSt-1BKlRkL85rlA1yZkr56BWbCvhKJrqwgsHMN3KI32wbnT0umhdGMsSWWG44A_AAU4oCYWJj"
        //const server_message = "ho_5N1Kup16z2J_aoR3MxLpxrM--gE-AFLz8-bhkIh_8cilJ2k3wlBxI5tG-aPV_-VNMoit3BFUK-8zO6cYpdAETrMqI8STeP2akP4qAmQ8A5nAFshWJUpU3NfznjqXFTFPMQRJAaV9Ga-xnDUXd7KTkW18gQeoI_QWXN9xgYaFJHsYTVOYXoWKkoOwbHfurl9tNesy7DhgOnFvBH7rxH3-i3Xcl4lPuHtFFlgNCLwR4r1V0wH9tFSGC30LmXpZOBLWWZ0IXIl5BBZ5mSCJJHS9UKiYIYAHjsDjpeMQaRm_0PA70Xqrlk1dLmlhrWSoX46pZQ3Bxp2bKxF38mtr3MQcAAO3RwD2P-EutfATHdQ2W1qQZuJyOjG255FSAsbBLIOFBcpYBCNIitdoxYe7baP6gI_A9LxyK4kP0kOXg17sQ8wQ="
        //const server_message = "GjLrN4JEUsjQgmesadkoPWbOblKFA2B_fbgFclxoW03GVBmt60hTg5I8TzpcuB6VAZffJkgztbfI5pETN-l-WAHbuTdN1azA6NI6d-oP3TOm-_sVanwq2zE35LJAMHhXQDdLpf3YxY3OCZfMCDfjz4hC8yU9KR4kawwKnnVj8cI_DjUG2M7pFJAR5VJ1j5yYmERTn_8S_vzxm6M6y0FGARx_J8HcjATeNkdiS9DCtte-1vCZa0UnhOpOf4IEEHl3AJ71NBsDbp8kEI4GanzhH3bPCqoWukPT_MToVe1pbROJkCKaxKwBu1PuMbF4e-hw4EtQuCJmb5l6-Zm7SkowBVYAAPfgo_zRAhkBivXxX0t0H33plYrN_7yKaDZIZiCMMyiuYabsvs_op4JKgD2hV-X1PPpUdrMZ-WVrZstLRiqr2_E="

        const message2 = opaque.client_register_finish_wasm(register_state, server_message)

        console.log(message2)

        const reqst2 = {
            "username": username,
            "client_request": message2,
            "auth_id": auth_id
        }
        await fetch(`http://localhost:${port}/auth/register/finish`, {
            method: 'POST', body: JSON.stringify(reqst2),
            headers: {
                'Content-Type': 'application/json'
            }
        })

    } catch (e) {
        console.log(e)
    }
}

async function login(username, password) {
    try {
        const { message: message1, state } = opaque.client_login_wasm(password)

        console.log(message1)
        console.log(state)

        // get message to server and get message back
        const reqst = {
            "username": username,
            "client_request": message1
        }
        const res = await fetch(`http://localhost:${port}/auth/login/start`, {
            method: 'POST', body: JSON.stringify(reqst),
            headers: {
                'Content-Type': 'application/json'
            }
        })
        const parsed = await res.json()
        const server_message = parsed.server_message
        const auth_id = parsed.auth_id
        const login_state = state
        console.log(auth_id)
        console.log(server_message)

        // pass 'abc'
        //const login_state = "Gg6GSd_2X9ccTkVZBatUyynmRM5CWBVh9j8Fsac2hQAAYoxXlNs3YTKM_4eq-Tr3hOM5TO1OZTaAgI7DYQIV4rhX-EomurCCwcw3cojfbBudPS6aF0YyxJZYbjgD8ABTigIAAMaJ77uRiMGm50uF6_VEFchFlKmwvKhhiUUsRhZhRl1fAEChX0fsJTWoEsS2bPTSt-1BKlRkL85rlA1yZkr56BWbCvhKJrqwgsHMN3KI32wbnT0umhdGMsSWWG44A_AAU4oCYWJj"
        //const server_message = "ho_5N1Kup16z2J_aoR3MxLpxrM--gE-AFLz8-bhkIh_8cilJ2k3wlBxI5tG-aPV_-VNMoit3BFUK-8zO6cYpdAETrMqI8STeP2akP4qAmQ8A5nAFshWJUpU3NfznjqXFTFPMQRJAaV9Ga-xnDUXd7KTkW18gQeoI_QWXN9xgYaFJHsYTVOYXoWKkoOwbHfurl9tNesy7DhgOnFvBH7rxH3-i3Xcl4lPuHtFFlgNCLwR4r1V0wH9tFSGC30LmXpZOBLWWZ0IXIl5BBZ5mSCJJHS9UKiYIYAHjsDjpeMQaRm_0PA70Xqrlk1dLmlhrWSoX46pZQ3Bxp2bKxF38mtr3MQcAAO3RwD2P-EutfATHdQ2W1qQZuJyOjG255FSAsbBLIOFBcpYBCNIitdoxYe7baP6gI_A9LxyK4kP0kOXg17sQ8wQ="
        //const server_message = "GjLrN4JEUsjQgmesadkoPWbOblKFA2B_fbgFclxoW03GVBmt60hTg5I8TzpcuB6VAZffJkgztbfI5pETN-l-WAHbuTdN1azA6NI6d-oP3TOm-_sVanwq2zE35LJAMHhXQDdLpf3YxY3OCZfMCDfjz4hC8yU9KR4kawwKnnVj8cI_DjUG2M7pFJAR5VJ1j5yYmERTn_8S_vzxm6M6y0FGARx_J8HcjATeNkdiS9DCtte-1vCZa0UnhOpOf4IEEHl3AJ71NBsDbp8kEI4GanzhH3bPCqoWukPT_MToVe1pbROJkCKaxKwBu1PuMbF4e-hw4EtQuCJmb5l6-Zm7SkowBVYAAPfgo_zRAhkBivXxX0t0H33plYrN_7yKaDZIZiCMMyiuYabsvs_op4JKgD2hV-X1PPpUdrMZ-WVrZstLRiqr2_E="

        const { message: message2, session } = opaque.client_login_finish_wasm(login_state, server_message)

        console.log(message2)

        const reqst2 = {
            "username": username,
            "client_request": message2,
            "auth_id": auth_id
        }
        const res2 = await fetch(`http://localhost:${port}/auth/login/finish`, {
            method: 'POST', body: JSON.stringify(reqst2),
            headers: {
                'Content-Type': 'application/json'
            }
        })

        const session_res = await res2.text()
        console.log(session)

    } catch (e) {
        if (e instanceof Error && e.name === "InvalidLogin") {
            console.log("IsError")
            console.log(e.message)
        }
        else {
            console.log(e)
        }
    }
}


// login("someperson", "garbage").catch(e => {
//     console.log(e)
// })

//regFinish()
//loginStart("garbage")
loginFinish()

// register("someperson", "garbage").catch(e => {
//     console.log(e)
// })

