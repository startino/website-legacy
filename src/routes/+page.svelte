<script lang="ts">
	import { base } from '$app/paths';
	import Header from '$lib/components/organisms/Header.svelte';
	import Footer from '$lib/components/organisms/Footer.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import ClientCarousel from '$lib/components/organisms/ClientCarousel.svelte';
	import ChapterMenu from '$lib/components/organisms/ChapterMenu.svelte';
	import { chapters } from './chapters';
	import { onMount } from 'svelte';
	import { tsParticles } from 'tsparticles-engine';
	import { loadCustom } from './tsparticles-custom-import'; // correct path format?
	import { tsparticlesOptions } from './tsparticles-options';
	import { inview } from 'svelte-inview';
	import type { Options } from 'svelte-inview';
	import { fade, slide } from 'svelte/transition';
	import TransitionElement, {
		type TransitionOptions
	} from '$lib/components/organisms/TransitionElement.svelte';
	import Icon from '$lib/components/atoms/Icon.svelte';
	import ClientCard from './ClientCard.svelte';

	let scrollY: number;

	const inviewOptions: Options = {
		rootMargin: '-10%',
		unobserveOnEnter: false
	};

	const leftSlidePreset: TransitionOptions = {
		delay: 100,
		duration: 300,
		transition: 'fly',
		x: -100
	};
	const rightSlidePreset = {
		delay: 100,
		duration: 300,

		transition: 'fly',
		x: 100
	};

	const clientCards = [
		{
			index: 0,
			img_path: 'fav_icon.png',
			name: 'John Mackadoo',
			company: 'Flyers Go Crazy',
			body: "I know there's a way i can type loris paragraphs in vscode but I forgot the shortcut, if you could lmk, would be appreciated.",
			vid_path: 'client_websites/ggsoccer_whole_dark.webm'
		},
		{
			index: 1,
			img_path: 'fav_icon.png',
			name: 'John Mackadoo',
			company: 'Flyers Go Crazy',
			body: "I know there's a way i can type loris paragraphs in vscode but I forgot the shortcut, if you could lmk, would be appreciated.",
			vid_path: 'client_websites/ggsoccer_whole_dark.webm'
		},
		{
			index: 2,
			img_path: 'fav_icon.png',
			name: 'John Mackadoo',
			company: 'Flyers Go Crazy',
			body: "I know there's a way i can type loris paragraphs in vscode but I forgot the shortcut, if you could lmk, would be appreciated.",
			vid_path: 'client_websites/ggsoccer_whole_dark.webm'
		},
		{
			index: 3,
			img_path: 'fav_icon.png',
			name: 'John Mackadoo',
			company: 'Flyers Go Crazy',
			body: "I know there's a way i can type loris paragraphs in vscode but I forgot the shortcut, if you could lmk, would be appreciated.",
			vid_path: 'client_websites/ggsoccer_whole_dark.webm'
		}
	];

	onMount(() => {
		const options = tsparticlesOptions;
		loadCustom(tsParticles);
		tsParticles.load('tsparticles-hero', options);
		tsParticles.load('tsparticles-client-carousel', options);
	});

	onMount(() => {});
</script>

<svelte:window bind:scrollY />

<Header />
<ChapterMenu {chapters} />

<main
	class="text-center border-b w shadow-2xl border-primary-light/40 dark:border-primary-dark/40 flex flex-col items-stretch"
>
	<!--Hero-->
	<section
		id="hero"
		class="grow py-32 h-screen sm:py-34 md:py-44 px-4 sm:px-6 md:px-8 grid justify-items-center space-y-12 relative"
	>
		<div id="tsparticles-hero" class="w-full h-full absolute -z-10" />
		<div class="grid justify-items-center space-y-12 h-fit w-full mx-auto self-center">
			<div class="w-fit">
				<h1 class="display-large">Futino</h1>
			</div>

			<div class="w-fit">
				<h3
					class="title-small md:headline-small overflow-hidden border-r-1 border-transparent whitespace-nowrap animate-typingsubtitle"
				>
					Launch Your Business's Online Presence with Confidence And Trust
				</h3>
			</div>

			<div class="gap-y-4 gap-x-4 grid grid-cols-2 sm::grid-cols-2">
				<a href="{base}/about">
					<Button class="w-full h-full">
						<p class="title-medium p-2">Check Out Pricing!</p>
					</Button>
				</a>
				<a href="{base}/contact">
					<Button class="w-full h-full">
						<p class="title-medium p-2">Contact Us!</p>
					</Button>
				</a>
			</div>
		</div>
	</section>

	<!--Big-Clients Slideshow-->
	<section id="slideshow" class="grid self-end -mx-6 z-10">
		<div id="tsparticles-client-carousel" class="w-full h-fit absolute -z-10" />
		<ClientCarousel />
	</section>

	<!--Journey Section-->
	<section
		id="journey"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center relative"
	>
		<!--Transition & parallax image, like pineview.io's one-->
		<div
			class="w-full h-full absolute top-0 bg-gradient-to-b from-transparent via-white/90 dark:via-black/90 to-transparent"
			style:transform={`translate3d(0, ${-scrollY * 5}px, 0)`}
		/>
		<!--Background image for journey section. Purpose is to blend with the transition image.-->
		<div
			class="bg-gradient-to-t from-white/50 dark:from-black/50 from-50% to-transparent -z-20 h-full w-full absolute"
		/>
		<h1 class="display-large py-12" in:fade>Areas of Expertise</h1>

		{#each chapters as { inView, chapterNumber, title, id, content }}
			<!--Chapter Setion-->
			<div {id} class="flex flex-col place-items-center py-24 relative overflow-hidden">
				<h1
					class="display-small p-4 font-extrabold tracking-wide transition-all duration-700 {inView
						? ' text-transparent bg-clip-text bg-gradient-to-r from-primary-light to-secondary-light dark:from-primary-dark dark:to-secondary-dark'
						: 'text-surface-on-light dark:text-surface-on-dark'}"
				>
					{title}
				</h1>

				<!--Center Line-->
				<TransitionElement transition="slide" class="h-full absolute left-1/2">
					<!--Background line-->
					<div class="border-l-2 border-white dark:border-black h-full absolute top-24 -z-10" />
					<!--Glow Line. Transition element not working on slide. tried debugging for a long time, no avail. this works though.-->
					<div
						in:slide={{ duration: 2000, axis: 'y', delay: 300 }}
						class="border-l-4 border-b-4 border-primary-light dark:border-primary-dark h-full top-24 absolute left-1/2 -z-20 opacity-100 blur-sm"
					/>
				</TransitionElement>

				<!--Circle-->
				<div
					class="h-14 w-14 rounded-full bg-surface-light dark:bg-surface-dark flex justify-self-center relative"
					use:inview={inviewOptions}
					on:inview_enter={(event) => {
						inView = true;
					}}
				>
					<h1
						class="headline-medium font-bold text-surface-on-light dark:text-surface-on-dark self-center mx-auto"
					>
						{chapterNumber}
					</h1>
					<!--Circle Glow Effect-->
					<TransitionElement transition="fade" delay={300} duration={500} class="-z-10">
						<div
							class="absolute -z-10 -inset-1 bg-gradient-to-r from-primary-dark to-secondary-dark animate-spin rounded-full blur transition-all"
						/>
					</TransitionElement>
				</div>
				<!--Content-->
				<div class="grid grid-cols-1 sm:grid-cols-2 gap-12 max-w-7xl justify-items-center py-8">
					{#each content as { title, body }}
						<!--Graphic Image-->
						<TransitionElement presetOptions={leftSlidePreset} class="w-full">
							<div
								class="bg-surface-variant-dark h-48 w-full sm:justify-self-end"
							/></TransitionElement
						>
						<!-- Title and Paragraph-->
						<TransitionElement presetOptions={rightSlidePreset} class="w-full">
							<div class="flex flex-col max-w-md text-left">
								<h2 class="display-small max-w-3xl">{title}</h2>
								<h3 class="body-medium max-w-3xl">
									{body}
								</h3>
							</div>
						</TransitionElement>
					{/each}
				</div>
			</div>
		{/each}
	</section>

	<!--Analytics Snippet-->
	<section
		id="analytics"
		class="grow px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 bg-primary-light/20 dark:bg-primary-dark/10"
	>
		<TransitionElement transition="slide" duration={200} delay={100} once={true}>
			<div class="flex flex-wrap md:flex-row px-4 justify-around text-center">
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
		</TransitionElement>
	</section>

	<!--Clients Section-->
	<section
		id="clients"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center"
	>
		<TransitionElement transition="fade" duration={500}>
			<h1 class="display-large">Our Clients</h1>
			<div class="flex flex-wrap gap-12 items-center justify-items-center max-w-7xl overflow-clip">
				{#each clientCards as { index, name, company, body, vid_path }}
					<ClientCard {name} {company} {body} {vid_path} />{/each}
			</div>
		</TransitionElement>
	</section>

	<section
		id="contact"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center"
	>
		<TransitionElement transition="fade" duration={500}>
			<h1 class="display-large">We'd Love to Hear From You</h1>

			<div
				class="grid grid-cols-1 lg:grid-cols-2 gap-6 mx-auto max-w-7xl items-center justify-items-center"
			>
				<!--PM Option-->
				<div class="flex p-8 shadow-lg shadow-black/40 space-y-12 flex-col max-w-3xl">
					<div class="text-left space-y-10">
						<h2 class="display-medium font-extrabold">Give us a PM</h2>
						<p class="title-medium">
							Send us message on one of these platforms. We'll get back to you within a couple
							hours.
						</p>
						<div class="flex-1 flex flex-col space-y-5 h-50">
							<div class="grid grid-cols-4 grid-rows-3 gap-4">
								<!--Phone number-->
								<div
									class="col-span-1 rounded-full bg-surface-variant-light w-min h-min ml-auto p-5"
								>
									<Icon icon="phone" height="32px" width="32px" />
								</div>

								<a class="col-span-3 flex items-center" href="tel:9133600394">
									<h1 class="title-small sm:title-large pl-2">+852 9747 3013</h1>
								</a>
								<!--WhatsApp-->
								<div
									class="col-span-1 rounded-full bg-surface-variant-light w-min h-min ml-auto p-5"
								>
									<Icon icon="email" height="32px" width="32px" fillColor="black" />
								</div>

								<a class="col-span-3 flex items-center" href="mailto:contact@futi.no">
									<h1 class="title-small sm:title-large pl-2">Futino Whatsapp</h1>
								</a>
								<!--Email-->
								<div
									class="col-span-1 rounded-full bg-surface-variant-light w-min h-min ml-auto p-5"
								>
									<Icon icon="instagram" height="32px" width="32px" fillColor="black" />
								</div>

								<a class="col-span-3 flex items-center" href="mailto:ggsoccercamps@gmail.com">
									<h1 class="title-small sm:title-large pl-2">@Futino</h1>
								</a>
								<!--Email-->
								<div
									class="col-span-1 rounded-full bg-surface-variant-light w-min h-min ml-auto p-5"
								>
									<Icon icon="email" height="32px" width="32px" fillColor="black" />
								</div>

								<a class="col-span-3 flex items-center" href="mailto:contact@futi.no">
									<h1 class="title-small sm:title-large pl-2">contact@futi.no</h1>
								</a>
							</div>
						</div>
					</div>
				</div>
				<!--Contact form Option-->
				<div class="flex p-8 shadow-lg shadow-black/40 space-y-12 flex-col w-full h-full">
					<div class="text-left space-y-10">
						<h2 class="display-medium font-extrabold">Contact Us</h2>
						<p class="title-medium">
							Feel free to us an email for any requests or questions. We'll get back to you within a
							couple hours.
						</p>
					</div>
					<form method="post" class="flex-1 flex flex-col space-y-5 h-64">
						<input
							class="border border-black/50 dark:border-white/50 p-1 dark:bg-black/5"
							type="text"
							name="name"
							id="name"
							placeholder="Name"
						/>
						<input
							class="border border-black/50 dark:border-white/50 p-1 dark:bg-surface-dark/5"
							type="text"
							name="email"
							id="email"
							placeholder="E-Mail"
						/>
						<textarea
							class="flex-1 border border-black/50 dark:border-white/50 p-1 dark:bg-black/5"
							name="message"
							id="message"
							placeholder="Message"
						/>
						<Button><input type="button" value="Send Email" /></Button>
					</form>
				</div>
			</div>
			<!-- Things like FAQ, requesting quotes, more about us, etc.-->
			<div class="flex flex-row space-x-4 py-8 w-fit mx-auto">
				{#each Array(2) as card}
					<div class="shadow-lg shadow-black/40 flex flex-col p-4">
						<h1 class="display-small font-bold">Title</h1>
						<Button class="body-medium font-semi-bold"
							><input type="button" value="Find Answers Now" /></Button
						>
					</div>
				{/each}
			</div>
		</TransitionElement>
	</section>

	<section
		id="about"
		class="grow py-24 sm:py-28 md:py-32 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center"
	>
		<TransitionElement transition="fade" duration={500} class="">
			<div class="flex flex-col max-w-7xl mx-auto place-items-center space-y-6">
				<h1 class="display-large py-12">Meet The Founders</h1>
				<div class="grid grid-cols-1 md:grid-cols-2 gap-x-4 gap-y-4">
					<!--Jorge's Card-->
					<div
						class="flex flex-col bg-surface-variant-light/20 shadow-lg dark:bg-surface-variant-dark/20 rounded-lg p-6"
					>
						<img src="" alt="Not found" class="rounded-full h-24 w-24 self-center bg-black" />
						<h2 class="display-small pt-2">Jorge Lewis</h2>
						<h3 class="title-small pb-4 text-gray-400">COO & Co-founder of Futino</h3>
						<h2 class="body-large">
							I noticed that making a website was either too expensive or too time consuming for
							everyone, including individuals, startups, even large businesses. I wanted to create a
							solution to these problems.
						</h2>
					</div>
					<!--Jonas' Card-->
					<div
						class="flex flex-col bg-surface-variant-light/20 shadow-lg dark:bg-surface-variant-dark/20 rounded-lg p-6"
					>
						<img src="" alt="Not found" class="rounded-full h-24 w-24 self-center bg-black" />
						<h2 class="display-small pt-2">Jonas Lindberg</h2>
						<h3 class="title-small pb-4 text-gray-400">CTO & Co-founder of Futino</h3>
						<h2 class="body-large">
							I noticed that making a website was either too expensive or too time consuming for
							everyone, including individuals, startups, even large businesses. I wanted to create a
							solution to these problems.
						</h2>
					</div>
				</div>
				<Button>
					<h1 class="p-2">Learn More</h1>
				</Button>
			</div>
		</TransitionElement>
	</section>

	<section
		id="calltoaction"
		class="grow py-24 sm:py-28 md:py-32 shadow-lg px-4 sm:px-6 md:px-8 grid space-y-12 border-secondary-light/20 dark:border-secondary-dark/20 justify-items-center"
	>
		<TransitionElement transition="fade" duration={300}>
			<div class="flex flex-col space-y-12">
				<h1 class="display-large">Let's Get Started</h1>
				<div class="gap-x-4 grid grid-cols-1 md:grid-cols-2">
					<a href="{base}/contact">
						<Button class="w-full">
							<p class="title-large p-4">Contact Us!</p>
						</Button>
					</a>
					<a href="{base}/about">
						<Button class="w-full">
							<p class="title-large p-4">Check Out Pricing!</p>
						</Button>
					</a>
				</div>
			</div></TransitionElement
		>
	</section>
</main>

<Footer />
