function fetchData() {
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(Math.floor(Math.random() * 100));
        }, Math.random() * 2000);
    });
}

function* dataGenerator() {
    for (let i = 0; i < 5; i++) {
        yield fetchData();
    }
}

async function processData() {
    for await (const data of dataGenerator()) {
        console.log('Received data:', data);
        // Perform operations with each fetched data here
    }
}

processData();
