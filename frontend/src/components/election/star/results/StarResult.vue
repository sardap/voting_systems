<script setup lang="ts">
import type { GenericElection, GenericElectionResult, StarResult } from '@/backend';
import { ref, type PropType } from 'vue';
import StarRankingTable from './StarRankingTable.vue';
import StarRunoffTable from './StarRunoffTable.vue';

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

const result = ref(props.result as StarResult)

</script>

<template>
  <h2>Score Round</h2>
  <StarRankingTable :options="result.options" :result="result" />
  <h2>Runoff</h2>
  <StarRunoffTable :options="result.options" :result="result" />
</template>

<style scoped></style>
