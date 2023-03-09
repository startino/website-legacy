<script lang="ts">
	import Button from "../atoms/Button.svelte";
	import PlanCard from "../molecules/PlanCard.svelte";

    export let plans = [
    { 
        label: "Partnership",
        cost: '3.8% of your revenue',
        features: [
           '50% off Hosting Plans',
           '50% off Marketing Plans',
           '90% off Initial Website Developement',
           '50% off Commissions',
        ],
		 value: 1,
	},
  ];

  export let activeTabValue = 0;
  
  const handleClick = (tabValue: number) => () => (activeTabValue = tabValue);
</script>

<div class="flex flex-col place-items-center w-[700px]">
    <h1 class="title-large text-center max-w-xl">
        Shooting for the stars?
    </h1>
    <h1 class="title-large pb-20 text-center max-w-lg"> <!--page is formatted quite weirdly and this text was overlappign with the cards so i had to add padding on bottom-->
        You'll reach the stars with us.
    </h1>
    <ul class="">
  {#each plans as plan}
      <li class='shadow-xl place-self-center {activeTabValue === plan.value ? 'active border-2 rounded-xl border-primary-light dark:border-primary-dark transition-all duration-500 scale-110' : 'hover:border-2 rounded-xl'}'>
        <button class="p-4 rounded-t-lg " on:click={handleClick(plan.value)} type="button" role="plan">
        <PlanCard plan={plan}/>
        </button>
      </li>
  {/each}
  </ul>
</div>