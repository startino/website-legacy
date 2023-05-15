<script lang="ts">
	import { base } from '$app/paths';
	import Header from '$lib/components/organisms/Header.svelte';
	import Footer from '$lib/components/organisms/Footer.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import ClientCarousel from '$lib/components/organisms/ClientCarousel.svelte';
	import ChapterMenu from '$lib/components/organisms/ChapterMenu.svelte';
	import { inview } from 'svelte-inview';
	import { fade } from 'svelte/transition';
	import type { ObserverEventDetails, ScrollDirection, Options } from 'svelte-inview';

	let scroll: any;
	let journeyInView: boolean;
	const handleJourneyDot = ({ detail }: CustomEvent<ObserverEventDetails>) => {
		console.log('handling change.');

		journeyInView = detail.inView;
	};

	const options: Options = {
		rootMargin: '-50%',
		unobserveOnEnter: false
	};
</script>

<svelte:window bind:scrollY={scroll} />

<Header />
<ChapterMenu />
<main
	class="text-center border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40 flex flex-col items-stretch"
>
	<!--Hero-->
	<section
		id="hero"
		class="grow py-32 h-screen sm:py-34 md:py-44 px-4 sm:px-6 md:px-8 grid justify-items-center space-y-12"
	>
		<div class="grid justify-items-center space-y-12 mx-auto">
			<h1 class="display-large">Futino</h1>

			<h3 class="text-2xl">Launch Your Business's Online Presence with Confidence And Trust</h3>

			<div class="space-x-6">
				<a href="{base}/about">
					<Button>
						<p>Check Out Pricing!</p>
					</Button>
				</a>
			</div>
		</div>
	</section>
	<!--Big-Clients Slideshow-->
	<section id="hero" class="grow grid space-y-2">
		<div class="px-4 sm:px-6 md:px-8">
			<h3 class="body-medium">Backed By Premium Companies</h3>
		</div>
		<ClientCarousel />
	</section>

	<!--Journey Section-->
	<section
		id="hero"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center"
		use:inview={options}
		on:inview_change={handleJourneyDot}
	>
		<h1 class="display-large">Areas of Expertise</h1>

		<!--Checkmark 1-->
		<div
			class="h-12 w-12 rounded-full bg-primary-light dark:bg-primary-dark flex justify-self-center relative"
		>
			<h1
				class="headline-medium text-primary-on-light dark:text-primary-on-dark self-center mx-auto"
			>
				1
			</h1>
			<div class="border-l-1 border-white/30 absolute left-1/2 h-[500px] -z-10" />
		</div>
		<!--Journey Path will be replaced with an svg later on-->

		<!--Journey Dot... Well sort of..
		{#if journeyInView}
			<div
				class="h-3 w-3 z-20 bg-tertiary-light dark:bg-tertiary-dark rounded-full absolute left-1/2 self-start"
				style:transform={`translateY( ${scroll * 1 - 1200}px)`}
			/>
		{/if}
		{scroll}
		-->
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-12 max-w-7xl justify-items-center">
			<!--Temp. image-->
			<div class="bg-surface-variant-dark h-42 w-52 sm:justify-self-end" />
			<div class="flex flex-col max-w-md text-left">
				<h2 class="display-small">We Lorem Ipsum</h2>
				<h3 class="body-medium">
					Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
					euismod euismod id ut est. In vel elit at lorem cursus porttitor.
				</h3>
			</div>
			<!--Temp. image-->
			<div class="bg-surface-variant-dark h-32 w-56 sm:justify-self-end" />
			<div class="flex flex-col max-w-md text-left">
				<h2 class="display-small">We Lorem Ipsum</h2>
				<h3 class="body-medium">
					Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
					euismod euismod id ut est.
				</h3>
			</div>
		</div>
		<!--Checkmark 2-->
		<div
			class="h-12 w-12 rounded-full bg-primary-light dark:bg-primary-dark flex justify-self-center relative"
		>
			<h1
				class="headline-medium text-primary-on-light dark:text-primary-on-dark self-center mx-auto"
			>
				2
			</h1>
			<div class="border-l-1 border-white/30 absolute left-1/2 h-[400px] -z-10" />
		</div>
		<!--Journey Path will be replaced with an svg later on-->

		<!--Journey Dot... Well sort of..
		{#if journeyInView}
			<div
				class="h-3 w-3 z-20 bg-tertiary-light dark:bg-tertiary-dark rounded-full absolute left-1/2 self-start"
				style:transform={`translateY( ${scroll * 1 - 1200}px)`}
			/>
		{/if}
		{scroll}
		-->
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-12 max-w-7xl">
			<!--Temp. image-->
			<div class="bg-surface-variant-dark h-42 w-52 sm:justify-self-end" />
			<div class="flex flex-col max-w-md text-left">
				<h2 class="display-small">We Lorem Ipsum</h2>
				<h3 class="body-medium">
					Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
					euismod euismod id ut est. In vel elit at lorem cursus porttitor.
				</h3>
			</div>
			<!--Temp. image-->
			<div class="bg-surface-variant-dark h-32 w-56 sm:justify-self-end" />
			<div class="flex flex-col max-w-md text-left">
				<h2 class="display-small">We Lorem Ipsum</h2>
				<h3 class="body-medium">
					Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
					euismod euismod id ut est.
				</h3>
			</div>
		</div>
	</section>

	<!--Analyitics Snippet-->
	<section
		id="hero"
		class="grow px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 bg-primary-light/20 dark:bg-primary-dark/10"
	>
		<div class="flex flex-row px-4 justify-around text-center">
			<div class="flex flex-col p-4">
				<h2 class="headline-large font-extrabold">50</h2>
				<h2 class="body-medium font-light">Sites Made</h2>
			</div>
			<div class="border-r border-white/20 my-4" />
			<div class="flex flex-col p-4 justify-self-end">
				<h2 class="headline-large font-extrabold ">21K</h2>
				<h2 class="body-medium font-light">Hours Used</h2>
			</div>
			<div class="border-r border-white/20 my-4" />
			<div class="flex flex-col p-4">
				<h2 class="headline-large font-extrabold">$200K</h2>
				<h2 class="body-medium font-light">Transacted</h2>
			</div>
			<div class="border-r border-white/20 my-4" />
			<div class="flex flex-col p-4">
				<h2 class="headline-large font-extrabold">99.9%</h2>
				<h2 class="body-medium font-light">Pure Waffle</h2>
			</div>
		</div>
	</section>

	<!--Clients Section-->
	<section
		id="hero"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20  justify-items-center"
	>
		<h1 class="display-large">Our Clients</h1>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-12 mx-auto max-w-7xl">
			{#each Array(6) as index}
				<div
					class="bg-surface-variant-light dark:bg-surface-variant-dark rounded-lg flex flex-col items-center p-6"
				>
					<img src="favicon.png" alt="Face" class="h-24 w-24 rounded-full" />
					<h1 class="headline-medium">John Doe, CEO of Phazor</h1>
					<h2 class="body-medium">
						Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
						euismod euismod id ut est. In vel elit at lorem cursus porttitor.
					</h2>
				</div>
			{/each}
		</div>
	</section>
</main>

<Footer />
