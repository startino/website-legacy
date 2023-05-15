<script lang="ts">
	import { base } from '$app/paths';
	import Header from '$lib/components/organisms/Header.svelte';
	import Footer from '$lib/components/organisms/Footer.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import ClientCarousel from '$lib/components/organisms/ClientCarousel.svelte';
	import ChapterMenu from '$lib/components/organisms/ChapterMenu.svelte';
</script>

<Header />

<main
	class="text-center border-b shadow-2xl border-primary-light/40 dark:border-primary-dark/40 flex flex-col items-stretch"
>
	<!--Hero-->
	<section
		id="hero"
		class="grow py-32 sm:py-34 md:py-44 shadow-lg px-4 sm:px-6 md:px-8 grid justify-items-center space-y-12 border-secondary-light/20 dark:border-secondary-dark/20"
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
</main>

<Footer />
