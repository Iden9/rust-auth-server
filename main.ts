






async function getpage() {
    const result = await fetch("https://www.baidu.com", {
        method: "GET"
    })
        .then(res =>res)

    console.log(result);
}
Response
getpage()