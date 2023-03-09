<script lang="ts">
	import { base } from '$app/paths';
	import Header from '$lib/components/organisms/Header.svelte';
	import Footer from '$lib/components/organisms/Footer.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
</script>

<Header />

<main
	class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40"
>
	<div class="grid justify-items-center space-y-6 mx-auto">
		<h1 class="display-large">Futino</h1>

		<h3 class="text-2xl">Launch Your Business's Online Presence with Confidence And Trust</h3>

		<div class="space-x-6">
			<a href="{base}/pricing">
				<Button>
					<p>Check the Pricing!</p>
				</Button>
			</a>
			<!-- <a href="{base}/about">
				<Button>
					<p>Who Are We?</p>
				</Button>
			</a> -->
		</div>
	</div>
</main>

<Footer />
