<script setup lang="ts">
import { type StvResult, get_single_transferable_vote_election_result, ElectionType, get_generic_election } from '@/backend';
import { onMounted, ref } from 'vue';
import PreferenceTally from '@/components/results/PreferenceTally.vue';
import EliminationTable from './EliminationTable.vue';
import WinnerTable from './WinnerTable.vue';
import VotesTable from '@/components/results/VotesTable.vue';
import type { GenericElection } from '@/backend';

const props = defineProps({
  election_id: {
    type: String,
    required: true,
  },
  api_key: {
    type: String,
    required: true,
  }
});


const loading = ref<boolean>(true);
const result = ref<StvResult | null>(null);
const election = ref<GenericElection | null>(null);
const exclude_candidates = ref<number[]>([]);
const exclude_candidate_options = ref<number[]>([]);
const exclude_selected = ref<number | null>(null);

async function refresh_results() {
  loading.value = true;
  console.log("Getting election result...");
  election.value = await get_generic_election(ElectionType.SingleTransferableVote, props.election_id);
  result.value = await get_single_transferable_vote_election_result(props.api_key, props.election_id, exclude_candidates.value);
  console.log(`Got election result`);
  exclude_candidate_options.value = election.value.options.map((_, i) => i);
  loading.value = false;
}

onMounted(async () => {
  await refresh_results();
});

function add_exclude_candidate() {
  const candidate_index = exclude_selected.value;
  if (candidate_index == null) {
    return;
  }

  if (exclude_candidates.value.includes(candidate_index)) {
    return;
  }

  exclude_candidate_options.value = exclude_candidate_options.value.filter(i => i != candidate_index);
  exclude_candidates.value.push(candidate_index);
}

</script>

<template>
  <div v-if="loading">
    <p>Loading...</p>
  </div>
  <div v-else>
    <div v-if="result && election">
      <h2>Total Votes: {{ result.votes.length }}</h2>
      <div v-if="election">
        <h3>Excluding</h3>
        <h4>Select which to exclude</h4>
        <select v-model="exclude_selected" @change="add_exclude_candidate">
          <option v-for="i in exclude_candidate_options" :value="i">{{
            election.options[i]
          }}</option>
        </select>
        <h4>Already selected exclude</h4>
        <ul>
          <li v-for="i in exclude_candidates">{{ election.options[i] }}</li>
        </ul>
        <button @click="refresh_results">Refresh</button>
      </div>
      <div v-if="result.votes.length > 0">
        <h3>Quota Size: {{ Math.floor(result.votes.length / result.elected_candidates.length) }}</h3>
        <br />
        <h3>Winners</h3>
        <WinnerTable :election_result="result" />
        <br />
        <h3>Rounds Table</h3>
        <EliminationTable :election="election" :election_result="result" />
        <br />
        <h3>Preference Tally</h3>
        <PreferenceTally :election_result="result" />
        <br />
        <h2>Vote Table</h2>
        <VotesTable :options="result.candidates" :votes="result.votes" />
      </div>
      <div v-else>
        <p>No votes yet</p>
      </div>
    </div>
    <div v-else>
      <p>Error</p>
    </div>
  </div>
</template>

<style scoped></style>
