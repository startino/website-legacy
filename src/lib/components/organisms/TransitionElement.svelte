<script context="module" lang="ts">
	// Want to try using this instead of props in the future to improve it.
	export type TransitionOptions = {
		top?: number;
		bottom?: number;
		once?: boolean;
		transition?: String;
		delay?: number;
		duration?: number;
		x?: number;
		y?: number;
	};
</script>

<script lang="ts">
	import { onMount } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import { fade, fly, slide } from 'svelte/transition';

	export let animation = 'none';
	export let animation_out = 'none; opacity: 0';
	export let css_animation = '';

	// Individual Options.
	export let once: boolean = false;
	export let transition: String = '';
	export let top: number = 0;
	export let bottom: number = 0;
	export let duration: number = 0;
	export let delay: number = 0;

	let defaultOptions: TransitionOptions = {
		once: false,
		transition: 'fade',
		top: 0,
		bottom: 0,
		delay: 0,
		duration: 300,
		x: 0,
		y: 0
	};
	export let presetOptions: TransitionOptions = defaultOptions;

	// This is the formatted version of the given prompts.
	const propOptions: TransitionOptions = {
		...defaultOptions,
		once: once,
		bottom: bottom,
		delay: delay,
		duration: duration,
		top: top,
		transition: transition
	};

	// Setup the finalizedOptions based on priority.
	const finalizedOptions: TransitionOptions = {
		// Lowest Priority
		...defaultOptions,
		// Middle Priority
		...propOptions,
		// Highest Priority
		...presetOptions
	};

	// Dispatch this event and update the check every frame.
	const dispatch = createEventDispatcher();
	$: dispatch('update', { observing: inView });

	// True if the element being observed is in the viewport
	let inView = true;

	// Generates a random ID to identify the element. There's probaby a better solution.
	let element: Element;

	function intersection_verify(element: Element) {
		// The space around that will trigger the animation.
		const rootMargin = `${-bottom}px 0px ${-top}px 0px`;

		// Configure Observer
		const observer = new IntersectionObserver(
			(entries) => {
				inView = entries[0].isIntersecting;
				if (inView && once) {
					observer.unobserve(element);
				}
			},
			{
				rootMargin
			}
		);
		// Begin observing
		observer.observe(element);

		return () => observer.unobserve(element);
	}

	onMount(() => {
		return intersection_verify(element);
	});
</script>

<div class={$$props.class} bind:this={element}>
	{#if inView}
		{#if finalizedOptions.transition == 'fade'}
			<div
				in:fade={{ duration: $$props.duration, delay: $$props.delay }}
				style="animation: {animation}; {css_animation}"
			>
				<slot />
			</div>
		{:else if finalizedOptions.transition == 'fly'}
			<div
				in:fly={{
					x: finalizedOptions.x,
					y: finalizedOptions.y,
					duration: finalizedOptions.duration,
					delay: finalizedOptions.delay
				}}
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
