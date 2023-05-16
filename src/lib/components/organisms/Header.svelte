<script lang="ts">
	import { base } from '$app/paths';
	import ThemeButton from '$lib/components/molecules/ThemeButton.svelte';
	import Logo from '$lib/components/atoms/Logo.svelte';
	import { slide } from 'svelte/transition';
	import { inview } from 'svelte-inview';
	import process from 'node:process';
	import { onMount } from 'svelte';

	let scroll: number;
	let hasScrolled: boolean = false; // If the user has scrolled at all => true
	let activeheaderClass = 'sticky top-0 z-40 flex-none w-full transition-colors duration-500';
	let largeHeaderClass =
		' py-10 text-primary-container-on-light dark:text-primary-container-on-dark';
	let miniHeaderClass =
		'py-6 backdrop-blur bg-surface-light dark:bg-surface-dark text-primary-container-on-light dark:text-primary-container-on-dark';

	// animation on scroll Solution 1
	/*
	
			function updateHeaderClass(scroll: number): String {
		if (scroll > 100) {
			hasScrolled = true;
			console.log('updateHeader');
			return miniHeaderClass;
		} else {
			return headerClass;
		}
	}
	$: headerClass = updateHeaderClass(scroll);
	*/
	// animation onscroll Solution 2
	onMount(() => {
		const header = document.querySelector('#header');

		window.addEventListener('scroll', () => {
			const currentScroll = window.pageYOffset;
			if (currentScroll > 100) {
				if (activeheaderClass != miniHeaderClass) {
					activeheaderClass = miniHeaderClass;
				}
			} else {
				if (activeheaderClass != largeHeaderClass) {
					activeheaderClass = largeHeaderClass;
				}
			}
		});
	});
</script>

<svelte:window bind:scrollY={scroll} />

<div
	id="header"
	class="{$$props.class} {activeheaderClass} fixed top-0 z-40 flex-none w-full transition-all duration-400"
>
	<div class="py-2 mx-5 border-secondary-light/10 dark:border-secondary-dark/10">
		<div class="flex relative items-center md:px-10 lg:px-20 xl:px-40 px-auto md:px-auto">
			<a class="flex overflow-hidden px-3 gap-3 justify-center" href="{base}/">
				<Logo />
				<p class="invisible pt-0.5 text-lg font-bold sm:visible">Futino</p>
			</a>
			<div class="flex items-center ml-auto">
				<nav class="body-large font-semibold tracking-wide">
					<ul class="flex m-auto space-x-8">
						<li>
							<a
								class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
								href="{base}/"
							>
								Home
							</a>
						</li>
						<li>
							<a
								class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
								href="{base}/contact"
							>
								Contact
							</a>
						</li>
						<li>
							<a
								class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
								href="{base}/about"
							>
								About
							</a>
						</li>

						<li>
							<a
								class="inline hover:text-tertiary-light dark:hover:text-tertiary-dark"
								href="{base}/pricing"
							>
								Pricing
							</a>
						</li>
						<li class="border-l border-secondary-light/20 dark:border-secondary-dark/20" />
						<li>
							<ThemeButton />
						</li>
					</ul>
				</nav>
			</div>
		</div>
	</div>
</div>
