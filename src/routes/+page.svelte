<script lang="ts">
	import { base } from '$app/paths';
	import Header from '$lib/components/organisms/Header.svelte';
	import Footer from '$lib/components/organisms/Footer.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import ClientCarousel from '$lib/components/organisms/ClientCarousel.svelte';
	import ChapterMenu from '$lib/components/organisms/ChapterMenu.svelte';
	import { onMount } from 'svelte';
	import { tsParticles } from 'tsparticles-engine';
	import { loadFull } from 'tsparticles';
	import { inview } from 'svelte-inview';
	import type { ObserverEventDetails, Options } from 'svelte-inview';
	import { fade } from 'svelte/transition';

	let scrollY: number;

	let chOneInView: boolean = false;
	const options: Options = {
		rootMargin: '-50%',
		unobserveOnEnter: true
	};
	const handleChange = ({ detail }: CustomEvent<ObserverEventDetails>) =>
		(chOneInView = detail.inView);

	//TODO create json object for tsparticle
	onMount(() => {
		loadFull(tsParticles);
		tsParticles.load('tsparticles-hero', {
			background: {},
			fpsLimit: 120,
			interactivity: {
				events: {
					onClick: {
						enable: true,
						mode: 'push'
					},
					resize: true
				},
				modes: {
					bubble: {
						distance: 400,
						duration: 2,
						opacity: 0.8,
						size: 40
					},
					push: {
						quantity: 10
					}
				}
			},
			particles: {
				color: {
					value: '#00e384'
				},
				links: {
					color: '#ffffff',
					distance: 100,
					enable: true,
					opacity: 0.5,
					width: 1
				},
				collisions: {
					enable: true
				},
				move: {
					direction: 'none',
					enable: true,
					outMode: 'bounce',
					random: false,
					speed: 3,
					straight: false
				},
				number: {
					density: {
						enable: true,
						area: 800
					},
					value: 7
				},
				opacity: {
					value: 0.5
				},
				shape: {
					type: 'circle'
				},
				size: {
					random: true,
					value: 5
				}
			},
			detectRetina: true,
			fullScreen: false
		});
		tsParticles.load('tsparticles-client-carousel', {
			background: {},
			fpsLimit: 120,
			interactivity: {
				events: {
					onClick: {
						enable: true,
						mode: 'push'
					},
					resize: true
				},
				modes: {
					bubble: {
						distance: 400,
						duration: 2,
						opacity: 0.8,
						size: 40
					},
					push: {
						quantity: 10
					}
				}
			},
			particles: {
				color: {
					value: '#00e384'
				},
				links: {
					color: '#ffffff',
					distance: 100,
					enable: true,
					opacity: 0.5,
					width: 1
				},
				collisions: {
					enable: true
				},
				move: {
					direction: 'none',
					enable: true,
					outMode: 'bounce',
					random: false,
					speed: 3,
					straight: false
				},
				number: {
					density: {
						enable: true,
						area: 400
					},
					value: 2
				},
				opacity: {
					value: 0.5
				},
				shape: {
					type: 'circle'
				},
				size: {
					random: true,
					value: 5
				}
			},
			detectRetina: true,
			fullScreen: false
		});
	});
</script>

<svelte:window bind:scrollY />

<Header />
<ChapterMenu />

<main
	class="text-center border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40 flex flex-col items-stretch"
>
	<!--Hero-->
	<section
		id="hero"
		class="grow py-32 h-screen sm:py-34 md:py-44 px-4 sm:px-6 md:px-8 grid justify-items-center space-y-12 relative"
	>
		<div id="tsparticles-hero" class="w-full h-full absolute -z-10" />
		<div class="grid justify-items-center space-y-12 mx-auto h-fit self-center">
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
	<section id="hero" class="grid self-end -mx-6">
		<div id="tsparticles-client-carousel" class="w-full h-fit absolute -z-10" />
		<ClientCarousel />
	</section>

	<!--Journey Section-->
	<section
		id="hero"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center relative"
	>
		<!--Going to be the transition & parallax image, like pineview.io's one-->
		<div
			class="w-full h-full absolute -z-10 top-0 bg-gradient-to-b from-black/5 via-black/90 to-black/5"
			style:transform={`translate3d(0, ${-scrollY * 1.1}px, 0)`}
		/>
		<!--Background image for journey section. Purpose is to blend with the transition image.-->
		<div
			class="bg-gradient-to-t from-black/50 from-50% to-transparent -z-20 h-full w-full absolute"
		/>
		<h1 class="display-large py-12">Areas of Expertise</h1>

		{#each Array(3) as chapter}
			<!--Chapter 1-->
			<div class="flex flex-col place-items-center py-12">
				<h1
					class="display-small p-4 font-extrabold tracking-wide transition-all duration-700 {chOneInView
						? ' text-transparent bg-clip-text bg-gradient-to-r from-primary-light to-secondary-light dark:from-primary-dark dark:to-secondary-dark'
						: 'text-surface-on-light dark:text-surface-on-dark'}"
				>
					Design
				</h1>

				<!--Circle and line-->
				<div
					class="h-14 w-14 rounded-full bg-surface-light dark:bg-surface-dark flex justify-self-center relative"
					use:inview={options}
					on:inview_change={handleChange}
				>
					<!--Circle Glow Effect-->
					{#if chOneInView === true}
						<div
							in:fade
							class="absolute -z-10 -inset-1 bg-gradient-to-r from-primary-dark to-secondary-dark animate-spin rounded-full blur opacity-100 transition-all"
						/>
					{/if}
					<h1
						class="headline-medium font-bold text-surface-on-light dark:text-surface-on-dark self-center mx-auto"
					>
						1
					</h1>
					<div class="border-l-1 border-white/30 absolute left-1/2 h-[450px] -z-10" />
				</div>
				<!--Content-->
				<div class="grid grid-cols-1 sm:grid-cols-2 gap-12 max-w-7xl justify-items-center py-8">
					<!--Temp. image-->
					<div class="bg-surface-variant-dark h-42 w-full sm:justify-self-end" />
					<div class="flex flex-col max-w-md text-left">
						<h2 class="display-small">We Lorem Ipsum</h2>
						<h3 class="body-medium">
							Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
							euismod euismod id ut est. In vel elit at lorem cursus porttitor.
						</h3>
					</div>
					<!--Temp. image-->
					<div class="bg-surface-variant-dark h-32 w-full sm:justify-self-end" />
					<div class="flex flex-col max-w-md text-left">
						<h2 class="display-small">We Lorem Ipsum</h2>
						<h3 class="body-medium">
							Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis nec neque vitae purus
							euismod euismod id ut est.
						</h3>
					</div>
				</div>
			</div>
		{/each}
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
				<h2 class="headline-large font-extrabold">21K</h2>
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
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center"
	>
		<h1 class="display-large">Our Clients</h1>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-12 mx-auto max-w-7xl">
			{#each Array(6) as index}
				<div
					class="bg-surface-variant-light/20 shadow-lg dark:bg-surface-variant-dark/20 rounded-lg max-w-3xl flex flex-col items-center p-6"
				>
					<!--<iframe
						src="https://ggsoccer.futi.no/"
						frameborder="0"
						class="w-fit h-auto aspect-[16/9]"
					/>-->
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
