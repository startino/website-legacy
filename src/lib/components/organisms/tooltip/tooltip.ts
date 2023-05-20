import Tooltip from './Tooltip.svelte';

export function tooltip(element: Element, content: string) {
	let tooltipComponent: Tooltip;
	function mouseOver(event) {
		tooltipComponent = new Tooltip({
			props: {
				content: content,
				x: event.pageX,
				y: event.pageY
			},

			target: document.body
		});
	}
	function mouseMove(event) {
		tooltipComponent.setPos(event.pageX, event.pageY);
	}
	function mouseLeave() {
		tooltipComponent.$destroy();
	}

	element.addEventListener('mouseover', mouseOver);
	element.addEventListener('mouseleave', mouseLeave);
	element.addEventListener('mousemove', mouseMove);

	return {
		destroy() {
			element.removeEventListener('mouseover', mouseOver);
			element.removeEventListener('mouseleave', mouseLeave);
			element.removeEventListener('mousemove', mouseMove);
		}
	};
}
