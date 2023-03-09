<script lang="ts">
	import Button from '../atoms/Button.svelte';
	import PlanCard from '../molecules/PlanCard.svelte';

	export let plans = [
		{
			label: 'Basic',
			unit: '$',
			cost: 29,
			note: '/mo',
			features: ['Hosting', 'Customer Support'],
			value: 1
		},
		{
			label: 'Standard',
			unit: '$',
			cost: 87,
			note: '/mo',
			features: ['Hosting', 'Priority Customer Suport', '2 hours Maintenance Work'],
			value: 2
		},
		{
			label: 'Premium',
			unit: '$',
			cost: 261,
			note: '/mo',
			features: [
				'Hosting',
				'Priority Customer Suport',
				'4 hours Maintenance Work',
				'2 hours Developement Work'
			],
			value: 3
		},
		{
			label: 'Premium',
			unit: '$',
			cost: 783,
			note: '/mo',
			features: [
				'Hosting',
				'Priority Customer Suport',
				'8 Maintenance Work',
				'8 hours Developement Work'
			],
			value: 3
		}
	];

	export let activeTabValue = 0;

	const handleClick = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<div class="grid space-y-10 place-items-center">
	<h1 class="title-large">
		Experience Lightning-Fast Website Hosting with Our Reliable and Secure Services.
	</h1>
	<ul class="flex flex-col xl:flex-row space-y-5 xl:space-x-5 xl:space-y-0">
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
