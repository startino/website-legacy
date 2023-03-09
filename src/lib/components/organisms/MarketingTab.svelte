<script lang="ts">
	import Button from '../atoms/Button.svelte';
	import PlanCard from '../molecules/PlanCard.svelte';

	export let plans = [
		{
			label: 'Basic',
			unit: '$',
			cost: 14,
			note: '/mo',
			features: ['Campaign Setup', 'Social Media Marketing'],
			value: 1
		},
		{
			label: 'Standard',
			unit: '$',
			cost: 28,
			note: '/mo',
			features: [
				'Campaign Setup',
				'Social Media Marketing',
				'Search Engine Optimization',
				'Regular Meetings: Yearly'
			],
			value: 2
		},
		{
			label: 'Business',
			unit: '$',
			cost: 85,
			note: '/mo',
			features: [
				'Campaign Setup',
				'Social Media Marketing',
				'Search Engine Optimization',
				'Regular Meetings: Biannually',
				'Consulting & strategy development'
			],
			value: 3
		},
		{
			label: 'Enterprise',
			unit: '$',
			cost: 257,
			note: '/mo',
			features: [
				'Campaign Setup',
				'Social Media Marketing',
				'Search Engine Optimization',
				'Regular Meetings: Quarterly',
				'Consulting & strategy development',
				'Social Media Management'
			],
			value: 4
		}
	];

	export let activeTabValue = 0;

	const handleClick = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<div class="flex flex-col p-3 place-items-center">
	<h1 class="title-large pb-20 text-center max-w-2xl ">
		<!--page is formatted quite weirdly and this text was overlappign with the cards so i had to add padding on bottom-->
		Maximize Your Reach with Tailored Digital Marketing.
	</h1>
	<ul class="flex space-x-10 text-center">
		{#each plans as plan}
			<li>
				<button
					class="inline-block p-4 rounded-t-lg shadow-xl bg-surface-variant-dark/20 {activeTabValue ===
					plan.value
						? 'active border-2 rounded-xl border-primary-light dark:border-primary-dark transition-all duration-500 scale-110'
						: 'hover:border-2 rounded-xl'}"
					on:click={handleClick(plan.value)}
					type="button"
					role="plan"
					aria-controls="profile"
					aria-selected="false"
				>
					<PlanCard {plan} />
				</button>
			</li>
		{/each}
	</ul>
</div>
