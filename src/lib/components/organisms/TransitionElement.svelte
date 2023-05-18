<script context="module" lang="ts">
	// Want to try using this instead of props in the future to improve it.
	export type TransitionOptions = {
		top: number;
		bottom: number;
		once: boolean;
	};
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import { fade, fly, slide } from 'svelte/transition';

	export let animation = 'none';
	export let animation_out = 'none; opacity: 0';
	export let css_animation = '';

	export let once = true;
	export let top = 0;
	export let bottom = 0;

	// cute litle reactive dispatch to get if is observing :3
	const dispatch = createEventDispatcher();
	// @ts-ignore
	$: dispatch('update', { observing: inView });

	// be aware... he's looking...
	let inView = true;

	// for some reason the 'bind:this={box}' on div stops working after npm run build... so... workaround time >:|
	const elementID = `random_id_-${Math.random()}`;

	/// current in experimental support, no support for IE (only Edge)
	/// see more in: https://developer.mozilla.org/en-US/docs/Web/API/IntersectionObserver
	/**
	 * @param {Element | null} element
	 */
	function intersection_verify(element: Element | null) {
		// bottom left top right
		const rootMargin = `${-bottom}px 0px ${-top}px 0px`;

		const observer = new IntersectionObserver(
			(entries) => {
				inView = entries[0].isIntersecting;
				if (inView && once) {
					// @ts-ignore
					observer.unobserve(element);
				}
			},
			{
				rootMargin
			}
		);

		// @ts-ignore
		observer.observe(element);
		// @ts-ignore
		return () => observer.unobserve(element);
	}

	/// Fallback in case the browser not have the IntersectionObserver
	/**
	 * @param {HTMLElement | null} box
	 */
	function bounding_verify(box: HTMLElement | null) {
		// @ts-ignore
		const c = box.getBoundingClientRect();
		inView = c.top + top < window.innerHeight && c.bottom - bottom > 0;

		if (inView && once) {
			// @ts-ignore
			window.removeEventListener('scroll', verify);
		}

		// @ts-ignore
		window.addEventListener('scroll', bounding_verify);
		// @ts-ignore
		return () => window.removeEventListener('scroll', bounding_verify);
	}

	onMount(() => {
		// for some reason the 'bind:this={box}' on div stops working after npm run build... so... workaround time >:|
		const box = document.getElementById(elementID);

		if (IntersectionObserver) {
			return intersection_verify(box);
		} else {
			return bounding_verify(box);
		}
	});
</script>

<div id={elementID} class={$$props.class}>
	{#if inView}
		{#if $$props.transition == 'fade'}
			<div
				in:fade={{ duration: $$props.duration, delay: $$props.delay }}
				style="animation: {animation}; {css_animation}"
			>
				<slot />
			</div>
		{:else if $$props.transition == 'fly'}
			<div
				in:fly={{ x: $$props.x, y: $$props.y, duration: $$props.duration, delay: $$props.delay }}
				style="animation: {animation}; {css_animation}"
			>
				<slot />
			</div>

			<!--Default to fade animation-->
		{:else}
			<div
				in:fade={{ duration: $$props.duration, delay: $$props.delay }}
				style="animation: {animation}; {css_animation}"
			>
				<slot />
			</div>
		{/if}
	{:else}
		<div style="animation: {animation_out}; {css_animation}">
			<slot />
		</div>
	{/if}
</div>
