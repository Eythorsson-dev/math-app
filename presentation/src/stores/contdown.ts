import { readable } from 'svelte/store';

export function createCountdown(numberOfSeconds: number, callback: Function) {
	if (numberOfSeconds > 60 * 60) {
		throw new Error("createCountdown only supports a maximum of 60 minutes");
	}

	if (numberOfSeconds <= 0) {
		throw new Error("createCountdown numberOfSeconds muse be greater than 0");
	}


	function format(numberOfSeconds: number) {
		let minutes = Math.floor(numberOfSeconds / 60);
		let seconds = numberOfSeconds % 60

		return [
			minutes.toString().padStart(2, "0"),
			seconds.toString().padStart(2, "0"),
		].join(":")
	}

	function getSecondsBetween(start: Date, end: Date): number {
		return Math.round((end.getTime() - start.getTime()) / 1000);
	}

	const $start = new Date();

	return readable(format(numberOfSeconds), function start(set) {
		const interval = setInterval(() => {
			const $now = new Date();
			const seconds = getSecondsBetween($start, $now);

			if (seconds >= numberOfSeconds) {
				clearInterval(interval);
				callback();

				set(format(0));
				return;
			}

			set(format(numberOfSeconds - seconds));
		}, 1000);

		return function stop() {
			clearInterval(interval);
		};
	});
}