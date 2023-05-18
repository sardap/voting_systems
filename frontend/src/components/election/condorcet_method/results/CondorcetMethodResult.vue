<script setup lang="ts">
import type { CondorcetMethodResult, GenericElection, GenericElectionResult } from '@/backend';
import { ref, type PropType } from 'vue';
import CondorcetMethodMatchups from './CondorcetMethodMatchups.vue';
import MatchedPairsTable from './MatchedPairsTable.vue';
import LockedInPairwise from './LockedInPairwise.vue';
import VotesTable from '@/components/results/VotesTable.vue';

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

const result = ref(props.result as CondorcetMethodResult)

</script>

<template>
  <h2>Match ups</h2>
  <CondorcetMethodMatchups :result="result" />
  <br />
  <div v-if="!result.condorcet_winner">
    <h2>No Condorcet winner need to run Ranked pairs</h2>
    <br />
    <h2>Ranked Pair list</h2>
    <MatchedPairsTable :result="result" />
    <br />
    <h2>Locked in pairwise victories</h2>
    <LockedInPairwise :result="result" />
    <div v-if="result.matched_pair_winner">
      <h2>Ranked Pair Winner: {{ result.options[result.matched_pair_winner] }} 🥇</h2>
    </div>
    <br />
    <div v-if="result.last_resort_winner">
      <h2>No Ranked Pair winner! Using AEC style pref voting</h2>
      <h2>Winner is: {{ result.options[result.last_resort_winner] }} 🥇</h2>
    </div>
  </div>
  <h2>Vote Table</h2>
  <VotesTable :options="result.options" :votes="result.votes" />
</template>

<style scoped></style>
