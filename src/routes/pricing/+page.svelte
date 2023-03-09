<script lang="ts">
	import { base } from '$app/paths';
	import Header from '$lib/components/organisms/Header.svelte';
	import Footer from '$lib/components/organisms/Footer.svelte';
	import ChoiceButton from '$lib/components/molecules/ChoiceButton.svelte';
	import OfferText from '$lib/components/molecules/OfferText.svelte';
	import Button from '$lib/components/atoms/Button.svelte';
	import HostingTab from '$lib/components/organisms/HostingTab.svelte';
	import MarketingTab from '$lib/components/organisms/MarketingTab.svelte';
	import PartnershipTab from '$lib/components/organisms/PartnershipTab.svelte';
	import Tabs from '$lib/components/organisms/Tabs.svelte';
	import CommisionTab from '$lib/components/organisms/CommisionTab.svelte';
	import PromotionsSlider from '$lib/components/molecules/PromotionsSlider.svelte';

	let tabs = [
		{ label: 'Hosting Plans', value: 1, component: HostingTab },
		{ label: 'Marketing Plans', value: 2, component: MarketingTab },
		{ label: 'Commission', value: 3, component: CommisionTab },
		{ label: 'Partnership', value: 4, component: PartnershipTab }
	];

	let promotions = [
		{ label: 'Monthly', value: 1, for: 'monthly', discount: '' },
		{ label: 'Biennially', value: 2, for: 'biennially', discount: '1 Month Free' },
		{ label: 'Yearly', value: 3, for: 'annuallyy', discount: '3 Months Free' }
	];
</script>

<Header />

<main
	class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40"
>
	<div class="grid justify-items-center space-y-12 mx-auto max-w-6xl">
		<h1 class="text-5xl">Adopt an online presence the way you want to.</h1>

		<h3 class="text-2xl">Consider some of our offers:</h3>

		<PromotionsSlider {promotions} />

		<Tabs {tabs} />
	</div>
</main>

<Footer />
