<script setup lang="ts">
import type { GenericElection, StvResult, StvRound } from '@/backend'
import type { PropType } from 'vue'

const props = defineProps({
  election_result: {
    type: Object as PropType<StvResult>,
    required: true
  },
  election: {
    type: Object as PropType<GenericElection>,
    required: true
  }
})

const quota = Math.max(
  1,
  Math.floor(props.election_result.votes.length / props.election_result.elected_candidates.length)
)

function get_vote_count_for_round(round: StvRound, candidate_index: number) {
  if (candidate_index in round.vote_counts) {
    return round.vote_counts[candidate_index]
  }

  return 0
}

function get_td_value(round_index: number, candidate_index: number) {
  let last_round: StvRound = { vote_counts: {}, elected_candidates: [], eliminated_candidates: [] }
  if (round_index > 0) {
    last_round = props.election_result.rounds[round_index - 1]
  }

  if (last_round.elected_candidates.includes(candidate_index)) {
    return '🏆'
  } else if (last_round.eliminated_candidates.includes(candidate_index)) {
    return '☠️'
  }

  const round = props.election_result.rounds[round_index]
  const vote_count = get_vote_count_for_round(round, candidate_index)

  let message = ''
  if (round.elected_candidates.includes(candidate_index)) {
    message = '🏆 '
  } else if (round.eliminated_candidates.includes(candidate_index)) {
    message = '☠️ '
  }

  const surplus = vote_count - quota
  if (surplus > 0) {
    message += `${vote_count} (${surplus})`
  } else {
    message += `${vote_count}`
  }

  return message
}

function get_td_class(round_index: number, candidate_index: number) {
  let last_round: StvRound = { vote_counts: {}, elected_candidates: [], eliminated_candidates: [] }
  if (round_index > 0) {
    last_round = props.election_result.rounds[round_index - 1]
  }

  if (last_round.elected_candidates.includes(candidate_index)) {
    return ''
  } else if (last_round.eliminated_candidates.includes(candidate_index)) {
    return ''
  }

  const round = props.election_result.rounds[round_index]

  if (round.elected_candidates.includes(candidate_index)) {
    return 'elected-this-round'
  } else if (round.eliminated_candidates.includes(candidate_index)) {
    return 'eliminated-this-round'
  }

  return ''
}
</script>

<template>
  <table>
    <tr>
      <th>Round</th>
      <th v-for="(candidate, i) in election.options" :key="i">{{ candidate }}</th>
    </tr>
    <tr v-for="round_idx in election_result.rounds.length" :key="round_idx">
      <td class="first-col">{{ round_idx + 1 }}</td>
      <td v-for="i in election.options.length" :class="get_td_class(round_idx, i)" :key="i">
        {{ get_td_value(round_idx, i) }}
      </td>
    </tr>
  </table>
</template>

<style scoped>
.elected-this-round {
  background-color: #74ef97;
}

.eliminated-this-round {
  background-color: #e74c3c;
}

/* table {
    border-collapse: collapse;
    border-left: 2px solid black;
    border-right: 2px solid black;
    border-top: 2px solid black;
}


th,
td {
    border: 1px solid black;
} */
</style>
