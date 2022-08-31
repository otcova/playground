import init, { fibonacci } from "rust"
import { bench } from "./bench"

init().then(() => {

	function jsFibonacci(n: number): number {
		if (n < 2) {
			return n
		} else {
			return jsFibonacci(n - 1) + jsFibonacci(n - 2)
		}
	}

	bench(jsFibonacci, fibonacci)

})