<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import QuotaPreferentialVicLabor2024Create from './QuotaPreferentialVicLabor2024Create.vue'
import type { QuotaPreferentialVicLabor2024CreateElection } from '@/backend'
import QuotaPreferentialVicLabor2024Ballot from './QuotaPreferentialVicLabor2024Ballot.vue'
import type { VoteOption } from '@/utils'

const emit = defineEmits<{
  (
    e: 'updated',
    value: {
      election: QuotaPreferentialVicLabor2024CreateElection
      bundles: { count: number; vote: number[] }[]
    }
  ): void
  (e: 'error', has_error: boolean): void
}>()

onMounted(() => {
  emit('error', true)
})

const create_election = ref<QuotaPreferentialVicLabor2024CreateElection>({
  title: 'test',
  elected_count: 5,
  candidates: [
    {
      name: '',
      is_female: true
    },
    {
      name: '',
      is_female: false
    }
  ],
  require_token: false,
  percent_female: 0.5
})

const create_election_changed = ref(0)

watch(create_election_changed, () => {
  options_for_ballots.value = create_election.value.candidates.map((candidate, index) => {
    return {
      index: index,
      name: candidate.name
    }
  })
})

const options_for_ballots = ref<VoteOption[]>([])

const ballots = ref<{ count: number; vote: number[] }[]>([])

const ballot_count = ref<number>(1)
const current_ballot = ref<number[]>([])

watch(
  [ballots, create_election],
  () => {
    if (ballots.value.length === 0) {
      emit('error', true)
    } else {
      emit('error', false)
    }
    emit('updated', {
      election: create_election.value,
      bundles: ballots.value
    })
  },
  { deep: true }
)

function add_ballots() {
  ballots.value.push({
    count: ballot_count.value,
    vote: current_ballot.value
  })
  ballot_count.value = 1
  current_ballot.value = []
  create_election_changed.value++
}

function ballot_to_string(ballot: number[]) {
  return ballot.map((index) => options_for_ballots.value[index].name).join(', ')
}

function remove_ballot_bundle(index: number) {
  ballots.value.splice(index, 1)
}
</script>

<template>
  <div>
    <div>
      <h2>Election Settings</h2>
      <QuotaPreferentialVicLabor2024Create
        @updated="
          (updated) => {
            create_election.candidates = updated.candidates
            create_election.elected_count = updated.elected_count
            create_election_changed++
          }
        "
      />
    </div>
    <div>
      <h2>Votes</h2>
      <QuotaPreferentialVicLabor2024Ballot
        :key="create_election_changed"
        :options="options_for_ballots"
        @updated="(updated) => (current_ballot = updated)"
      />
      <div>
        <label for="ballot_count">Ballot Count</label>
        <input type="number" min="1" max="200" v-model="ballot_count" />
      </div>
      <div>
        <button @click="add_ballots" :disabled="current_ballot.length === 0">Add Ballots</button>
      </div>
    </div>
  </div>
  <div>
    <h2>Existing Ballots</h2>
    <div v-for="(bundle, i) in ballots" :key="i">
      <p>
        Ballot: {{ ballot_to_string(bundle.vote) }} Count:{{ bundle.count }}
        <button @click="remove_ballot_bundle(i)">Remove</button>
      </p>
    </div>
  </div>
</template>

<style scoped>
label {
  margin-right: 5px;
}
</style>
