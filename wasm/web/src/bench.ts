function time(code) {
	let start = performance.now()
	code()
	let end = performance.now()
	return end - start
}


export function bench(...functions: ((index: number) => number)[]) {
	let sum = 0
	const times = Array(functions.length).fill(0)
	
	let startTime = performance.now()
	while (performance.now() - startTime < 1000 * functions.length) {
		for (let i = 0; i < functions.length; ++i) {
			const func = functions[i]
			times[i] += time(() => {
				for (let index = 0; index < 40; ++index) {
					sum += func(index)
				}
			})
		}
	}

	const maxTime = times.reduce((a, b) => Math.max(a, b))

	for (let i = 0; i < functions.length; ++i) {
		const percentageTime = Math.round(1000 * times[i] / maxTime) / 10;
		console.log(">", functions[i].prototype?.constructor.name, 
			"\ttime: ", percentageTime, "%",
			"\t", times[i], "ms")
	}
}