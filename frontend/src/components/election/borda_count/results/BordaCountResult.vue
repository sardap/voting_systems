<script setup lang="ts">
import type { BordaCountResult, GenericElection, GenericElectionResult } from '@/backend';
import { ref, type PropType } from 'vue';
import BordaCountRankingTable from '@/components/election/borda_count/results/BordaCountRankingTable.vue';

const props = defineProps({
  election: {
    type: Object as PropType<GenericElection>,
    required: true,
  },
  result: {
    type: Object as PropType<GenericElectionResult>,
    required: true,
  }
});

const result = ref(props.result as BordaCountResult)

</script>

<template>
  <h2>Borda Count result</h2>
  <BordaCountRankingTable :options="result.options" :borda_count_tally="result.vote_tally" />
</template>

<style scoped></style>
