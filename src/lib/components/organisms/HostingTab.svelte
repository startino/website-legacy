<script lang="ts">
	import Button from '../atoms/Button.svelte';
	import PlanCard from '../molecules/PlanCard.svelte';

	export let plans = [
		{
			label: 'Basic',
			cost: 29,
			features: ['Hosting', 'Customer Support'],
			value: 1
		},
		{
			label: 'Standard',
			cost: 87,
			features: ['Hosting', 'Priority Customer Suport', '1hr Maintenance Work'],
			value: 2
		},
		{
			label: 'Premium',
			cost: 261,
			features: [
				'Hosting',
				'Priority Customer Suport',
				'8hrs Maintenance Work',
				'8hrs Developement Work'
			],
			value: 3
		},
		{
			label: 'Premium',
			cost: 783,
			features: [
				'Hosting',
				'Priority Customer Suport',
				'8hrs Maintenance Work',
				'8hrs Developement Work'
			],
			value: 3
		}
	];

	export let activeTabValue = 0;

	const handleClick = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<div class="flex flex-col p-3 place-items-center">
	<h1 class="title-large pb-20 text-center max-w-2xl ">
		<!--page is formatted quite weirdly and this text was overlappign with the cards so i had to add padding on bottom-->
		Experience Lightning-Fast Website Hosting with Our Reliable and Secure Services. Our Edge Computing
		will never leave you on the edge.
	</h1>
	<ul class="flex space-x-10 text-center">
		{#each plans as plan}
			<li
				class="shadow-xl {activeTabValue === plan.value
					? 'active border-2 rounded-xl border-primary-light dark:border-primary-dark transition-all duration-500 scale-110'
					: 'hover:border-2 rounded-xl'}"
			>
				<button
					class="inline-block p-4 rounded-t-lg "
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
