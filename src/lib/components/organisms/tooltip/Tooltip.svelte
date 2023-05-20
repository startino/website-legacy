<script lang="ts">
	export let content = '';
	let isHovered = false;
	export let x: number;
	export let y: number;

	function mouseOver(event: { pageX: number; pageY: number }) {
		console.log('Mouse over');
		isHovered = true;
		x = event.pageX + 5;
		y = event.pageY + 5;
	}
	function mouseMove(event: { pageX: number; pageY: number }) {
		console.log('Mouse move');
		x = event.pageX + 5;
		y = event.pageY + 5;
	}
	function mouseLeave() {
		console.log('Mouse leave');
		isHovered = false;
	}

	export function setPos(newX: number, newY: number) {
		x = newX;
		y = newY;
		console.log(x, y);
		console.log(content);
	}
</script>

<div class="bg-gray-300 h-64 w-64 absolute z-50 top-[{y}px] left-[{x}px]">
	<h1 class="display-large">
		{content}
	</h1>
</div>
