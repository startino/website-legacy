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

    let tabs = [
    { label: "Hosting Plans",
		 value: 1,
		 component: HostingTab
		},
    { label: "Marketing Plans",
		 value: 2,
		 component: MarketingTab
		},
    { label: "Commission",
		 value: 3,
		 component: CommisionTab
		},
        { label: "Partnership",
		 value: 4,
		 component: PartnershipTab
		}
  ];
</script>

<Header />

<main
	class="py-40 px-4 border-b shadow-2xl sm:px-6 md:px-8 border-primary-light/40 dark:border-primary-dark/40"
>
	<div class="grid justify-items-center space-y-12 mx-auto max-w-6xl">
		<h1 class="text-5xl">
            Adopt an online presence the way you want to.
        </h1>

		<h3 class="text-2xl">
			Consider some of our offers:
		</h3>
        
        <!-- Billing Cycle -->
        <div class="flex flex-row rounded bg-black/10 p-3 space-x-1">
            <ChoiceButton name="billing" for="monthly" text="Monthly" />

            <ChoiceButton name="billing" for="biennially" text="Biennially">
                <OfferText class="pl-2">
                    1 Month free
                </OfferText>
            </ChoiceButton>

            <ChoiceButton name="billing" for="yearly" text="Yearly">
                <OfferText class="pl-2">
                    3 Months free
                </OfferText>
            </ChoiceButton>
        </div>

        <!-- Plans -->
        <Tabs {tabs}/>
        <!-- <div id="myTabContent">
            <div class="hidden p-4 rounded-lg bg-gray-50 dark:bg-gray-800" id="profile" role="tabpanel" aria-labelledby="profile-tab">
                <p class="text-sm text-gray-500 dark:text-gray-400">This is some placeholder content the <strong class="font-medium text-gray-800 dark:text-white">Profile tab's associated content</strong>. Clicking another tab will toggle the visibility of this one for the next. The tab JavaScript swaps classes to control the content visibility and styling.</p>
            </div>
            <div class="hidden p-4 rounded-lg bg-gray-50 dark:bg-gray-800" id="dashboard" role="tabpanel" aria-labelledby="dashboard-tab">
                <p class="text-sm text-gray-500 dark:text-gray-400">This is some placeholder content the <strong class="font-medium text-gray-800 dark:text-white">Dashboard tab's associated content</strong>. Clicking another tab will toggle the visibility of this one for the next. The tab JavaScript swaps classes to control the content visibility and styling.</p>
            </div>
        </div> -->
	</div>
</main>

<Footer />