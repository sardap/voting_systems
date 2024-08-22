<script setup lang="ts">
import { useRoute } from 'vue-router'
import { onMounted, ref, type PropType } from 'vue'
import {
  ElectionType,
  get_generic_election,
  type GenericElection,
  type GenericElectionResult,
  get_generic_election_result
} from '@/backend'
import ElectionResult from '@/components/results/ElectionResult.vue'
import ApprovalResult from '@/components/election/approval/results/ApprovalResult.vue'
import PrefElectionResult from '@/components/election/preferential_voting/results/PrefElectionResult.vue'
import StvElectionResult from '@/components/election/single_transferable_vote/results/StvElectionResult.vue'
import BordaCountResult from '@/components/election/borda_count/results/BordaCountResult.vue'
import StarResult from '@/components/election/star/results/StarResult.vue'
import CumulativeResult from '@/components/election/cumulative/results/CumulativeResult.vue'
import AntiPluralityResult from '@/components/election/anti_plurality/results/AntiPluralityResult.vue'
import SinglePartyResult from '@/components/election/single_party/results/SinglePartyResult.vue'
import ThreeTwoOneResult from '@/components/election/three_two_one/results/ThreeTwoOneResult.vue'
import CondorcetMethodResult from '@/components/election/condorcet_method/results/CondorcetMethodResult.vue'
import MajorityJudgmentResult from '@/components/election/majority_judgment/results/MajorityJudgmentResult.vue'
import ScoreResult from '@/components/election/score/results/ScoreResult.vue'
import UsualJudgmentResult from '@/components/election/usual_judgment/results/UsualJudgmentResult.vue'
import CandidateTable from '@/components/results/CandidateTable.vue'
import SingleNonTransferableResult from '@/components/election/single_non_transferable/results/SingleNonTransferableResult.vue'
import InfoCom from '@/components/InfoCom.vue'

const props = defineProps({
  election_type: {
    type: Object as PropType<ElectionType>,
    required: true
  }
})

const route = useRoute()
const election_id = route.params.id as string
const api_key = route.query.api_key as string | undefined

const loading = ref<boolean>(true)
const election = ref<GenericElection | null>(null)
const result = ref<GenericElectionResult | null>(null)

onMounted(async () => {
  loading.value = true
  console.log('Getting base election')
  election.value = await get_generic_election(props.election_type, election_id)
  console.log(`Got base election`)
  loading.value = false

  console.log('Getting election result')
  result.value = await get_generic_election_result(props.election_type, election_id, api_key)
  console.log(`got election result ${JSON.stringify(result.value)}`)
})
</script>

<template>
  <div>
    <h1>Election Result</h1>
    <br />
    <InfoCom :election_type="election_type" />
    <br />
    <div>
      <p v-if="loading">Loading..</p>
      <div v-else-if="election">
        <ElectionResult
          v-if="api_key"
          :api_key="api_key"
          :election_type="election_type"
          :election="election"
        />
        <h2>Candidates</h2>
        <CandidateTable :candidates="election.options" />
        <div v-if="result">
          <br />
          <h2>Total Votes: {{ result.vote_count }}</h2>
          <div v-if="result.vote_count > 0">
            <PrefElectionResult
              v-if="election_type == ElectionType.PreferentialVoting"
              :election="election"
              :result="result"
            />
            <StvElectionResult
              v-else-if="election_type == ElectionType.SingleTransferableVote && api_key"
              :election_id="election_id"
              :api_key="api_key"
            />
            <ScoreResult
              v-else-if="election_type == ElectionType.Score"
              :election="election"
              :result="result"
            />
            <MajorityJudgmentResult
              v-else-if="election_type == ElectionType.MajorityJudgment"
              :election="election"
              :result="result"
            />
            <ThreeTwoOneResult
              v-else-if="election_type == ElectionType.ThreeTwoOne"
              :election="election"
              :result="result"
            />
            <SinglePartyResult
              v-else-if="election_type == ElectionType.SingleParty"
              :election="election"
              :result="result"
            />
            <AntiPluralityResult
              v-else-if="election_type == ElectionType.AntiPlurality"
              :election="election"
              :result="result"
            />
            <CumulativeResult
              v-else-if="election_type == ElectionType.Cumulative"
              :election="election"
              :result="result"
            />
            <StarResult
              v-else-if="election_type == ElectionType.Star"
              :election="election"
              :result="result"
            />
            <BordaCountResult
              v-else-if="election_type == ElectionType.BordaCount"
              :election="election"
              :result="result"
            />
            <CondorcetMethodResult
              v-else-if="election_type == ElectionType.CondorcetMethod"
              :election="election"
              :result="result"
            />
            <UsualJudgmentResult
              v-else-if="election_type == ElectionType.UsualJudgment"
              :election="election"
              :result="result"
            />
            <ApprovalResult
              v-else-if="election_type == ElectionType.Approval"
              :election="election"
              :result="result"
            />
            <SingleNonTransferableResult
              v-else-if="election_type == ElectionType.SingleNonTransferable"
              :election="election"
              :result="result"
            />
          </div>
          <h2 v-else>No votes yet</h2>
        </div>
        <h2 v-else>Error Getting result probably because no votes yet</h2>
      </div>
    </div>
  </div>
</template>

<style></style>
