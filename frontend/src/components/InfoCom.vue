<script setup lang="ts">
import { ElectionType } from '@/backend'
import { ref, type PropType } from 'vue'

const props = defineProps({
  election_type: {
    type: Object as PropType<ElectionType>,
    required: true
  }
})

interface Info {
  title: string
  summary: string
  benefits: string[]
  issues: string[]
  link: string
  places_used_in: string[]
  election_type: ElectionType
}

const info = [
  {
    title: 'What is Approval Voting?',
    summary:
      'Approval Voting is a voting system where each voter can approve of as many candidates as they like. There is no ranking or weighting of candidates. The candidate with the most approvals wins the election.',
    benefits: ['Simplicity', 'Encourages honest voting'],
    issues: ['Limited expressiveness', 'Potential for ties'],
    link: 'https://en.wikipedia.org/wiki/Approval_voting',
    places_used_in: [],
    election_type: ElectionType.Approval
  },
  {
    title: 'What is Borda Count?',
    summary:
      "Borda Count is a voting system where candidates get points based on their ranking in each voter's preference list. The candidate with the highest total points wins.",
    benefits: ['Straightforward', 'Promotes consensus candidates', 'Encourages sincere voting'],
    issues: ['May not select the most preferred candidate'],
    link: 'https://en.wikipedia.org/wiki/Borda_count',
    places_used_in: ['🇳🇷 Nauru - National'],
    election_type: ElectionType.BordaCount
  },
  {
    title: 'What is Preferential Voting (AEC style)?',
    summary:
      'Preferential voting, also known as ranked-choice voting or instant-runoff voting, is a system where voters rank candidates in order of preference. If no candidate has a majority, the lowest-ranked candidate is eliminated, and their votes are redistributed according to the next preferences until a candidate has a majority.',
    benefits: ['Promotes consensus candidates', 'Encourages more diverse candidates'],
    issues: ['Can sometimes produce unexpected outcomes'],
    link: 'https://www.aec.gov.au/learn/preferential-voting.htm',
    places_used_in: [
      '🇦🇺 Australia - Federal',
      '🇮🇪 Ireland - Presidential',
      '🇵🇬 Papua New Guinea - National'
    ],
    election_type: ElectionType.PreferentialVoting
  },
  {
    title: 'What is Single Transferable Vote?',
    summary:
      'Single Transferable Vote (STV) is a voting system where voters rank candidates, allowing multiple candidates to be elected in one election. Candidates reach a quota to be elected, and surplus votes or votes from eliminated candidates are redistributed until enough candidates are elected.',
    benefits: [
      'Provides proportional representation',
      'Encourages a diverse range of candidates',
      'Allows for more voter choice',
      'Supports electing multiple candidates'
    ],
    issues: ['Can sometimes produce unexpected outcomes'],
    link: 'https://en.wikipedia.org/wiki/Single_transferable_vote',
    places_used_in: [
      '🇦🇺 Australia - Senate',
      '🇮🇪 Ireland - Parliamentary',
      '🇫🇯 Fiji - National (Not exactly but close)',
      '🇲🇹 Malta - National'
    ],
    election_type: ElectionType.SingleTransferableVote
  },
  {
    title: 'What is STAR Voting?',
    summary:
      'STAR Voting (Score Then Automatic Runoff) is a voting system where voters rate each candidate on a numerical scale (e.g., 0-5). The two candidates with the highest total scores proceed to an automatic runoff. In the runoff, the candidate who is preferred by more voters wins the election.',
    benefits: ['Expressive', 'Reduces strategic voting'],
    issues: ['Complexity', 'Potential for strategic manipulation'],
    link: 'https://en.wikipedia.org/wiki/STAR_voting',
    places_used_in: [],
    election_type: ElectionType.Star
  },
  {
    title: 'What is Cumulative voting?',
    summary:
      'Cumulative voting is a system where voters have multiple votes to distribute among candidates, allowing them to give more support to their preferred candidates.',
    benefits: [
      'Encourages minority representation',
      'Allows voters to express varied levels of support for different candidates'
    ],
    issues: ['Complexity', 'Vulnerable to manipulation'],
    link: 'https://en.wikipedia.org/wiki/Cumulative_voting',
    places_used_in: ['🇮🇱 Israel - Local'],
    election_type: ElectionType.Cumulative
  },
  {
    title: 'What is Anti-plurality Voting?',
    summary:
      "Anti-plurality voting, also known as plurality loser, disapproval voting or 'least-most' voting, is a voting system where voters can mark their disapproval for one candidate. The candidate with the most disapproval votes is eliminated, and the one with the least disapproval votes wins.",
    benefits: [
      'Discourages strategic voting',
      'Encourages voters to express their true preferences',
      'Favors candidates with broader appeal'
    ],
    issues: [
      'Limited expressiveness',
      'Can be influenced by tactical voting',
      'Less known and studied compared to other voting methods'
    ],
    link: 'https://en.wikipedia.org/wiki/Anti-plurality_voting',
    places_used_in: [],
    election_type: ElectionType.AntiPlurality
  },
  {
    title: 'What is One Party Elections?',
    summary:
      'One Party elections are a voting system where only one political party is allowed to run for office. In such a system, the ruling party would put forward a single candidate for each position, and voters could either approve or disapprove the candidate. In practice even if the voters disapprove of the candidate very little would happen.',
    benefits: ['Faster counting', 'Save Money on the ballots (Less Ink)'],
    issues: [
      'Lack of political diversity and choice for voters',
      'Imbalance of power',
      'Potential for corruption and authoritarian rule',
      'Limited accountability'
    ],
    link: 'https://en.wikipedia.org/wiki/Elections_in_the_Soviet_Union',
    places_used_in: [
      '🇨🇳 China - All levels',
      '🇰🇵 North Korea - All levels',
      '🇻🇳 Vietnam - All levels',
      '🇧🇳 Brunei - National',
      '🇬🇶 Equatorial Guinea - National',
      '🇪🇷 Eritrea - National',
      '🇰🇿 Kazakhstan - National',
      '🇱🇦 Laos - National'
    ],
    election_type: ElectionType.SingleParty
  },
  {
    title: 'What is 3-2-1 Voting?',
    summary:
      "3-2-1 Voting is a voting system where voters rate each candidate as 'Good', 'OK', or 'Bad'. The three candidates with the highest 'Good' scores advance to the next round, where the candidate with the fewest 'Bad' scores is eliminated. In the final round, the candidate with the most 'OK' scores wins the election.",
    benefits: [
      'Simple to understand',
      'Encourages honest voting',
      'Reduces strategic voting',
      'Favors consensus candidates'
    ],
    issues: [
      'May not always elect the most preferred candidate',
      'Less expressive than some other ranked systems'
    ],
    link: 'https://electowiki.org/wiki/3-2-1_voting',
    places_used_in: [],
    election_type: ElectionType.ThreeTwoOne
  },
  {
    title: 'What is the Condorcet winner or Ranked Pairs?',
    summary:
      'The Condorcet method is a voting system where voters rank candidates in order of preference. The winner is the candidate who, when compared to every other candidate, would win in a head-to-head matchup more often. In other words, the Condorcet winner is the candidate preferred by the majority of voters over any other candidate. If a Condorcet winner exists, they are elected. If not, the winner is determined by a method called Ranked Pairs.',
    benefits: [
      'Encourages honest voting',
      'Selects the candidate with the broadest support',
      'Favors consensus candidates',
      'Reduces strategic voting'
    ],
    issues: [
      "May not always produce a clear winner due to 'Condorcet paradox'",
      'More complex to count and understand',
      'Vulnerable to strategic manipulation in some cases'
    ],
    link: 'https://en.wikipedia.org/wiki/Condorcet_method',
    places_used_in: [],
    election_type: ElectionType.CondorcetMethod
  },
  {
    title: 'What is Majority Judgment?',
    summary:
      "Majority Judgment is a voting system where voters rate each candidate using words like 'Excellent', 'Good', 'OK', and so on. Candidates' ratings are compared, and the one with the best overall rating wins.",
    benefits: [
      'Encourages honest voting',
      'Reduces strategic voting',
      'Favors consensus candidates'
    ],
    issues: ['More complex than some other systems', 'Potential for ties'],
    link: 'https://en.wikipedia.org/wiki/Majority_judgment',
    places_used_in: [],
    election_type: ElectionType.MajorityJudgment
  },
  {
    title: 'What is Score Voting?',
    summary:
      'Score Voting, also known as range voting, is a voting system where voters give each candidate a score, usually within a predefined range (e.g., 0-10). The candidate with the highest average score wins the election.',
    benefits: [
      'Expressive',
      'Encourages honest voting',
      'Reduces strategic voting',
      'Favors consensus candidates'
    ],
    issues: ['Complexity', 'Potential for ties', 'Vulnerable to strategic manipulation'],
    link: 'https://en.wikipedia.org/wiki/Score_voting',
    places_used_in: [],
    election_type: ElectionType.Score
  },
  {
    title: 'What is Usual Judgment?',
    summary:
      "Usual Judgment is a hypothetical variant of Majority Judgment, where voters rate each candidate using words like 'Excellent', 'Good', 'OK', and so on. However, unlike Majority Judgment, Usual Judgment compares candidates' average ratings instead of their median ratings. The candidate with the highest average rating wins the election.",
    benefits: ['Expressive', 'Encourages honest voting', 'Favors consensus candidates'],
    issues: [
      'More complex than some other systems',
      'Potential for ties',
      'Vulnerable to strategic manipulation'
    ],
    link: 'https://en.m.wikipedia.org/wiki/Usual_judgment',
    places_used_in: [],
    election_type: ElectionType.UsualJudgment
  },
  {
    title: 'What is Single Non Transferable Vote?',
    summary:
      "Single Non-Transferable Vote (SNTV) is a voting method in which voters have one vote to cast in a multi-member district. The candidates with the most votes are elected. It's commonly used in multi-member district elections.",
    benefits: [
      'Simple for voters to understand and use',
      'Minority groups have a chance to gain representation',
      'Discourages negative campaigning as candidates compete for second and third preferences'
    ],
    issues: [
      'Does not always result in proportional representation',
      'Can lead to tactical voting',
      'May not provide a clear mandate for governance'
    ],
    link: 'https://en.wikipedia.org/wiki/Single_non-transferable_vote',
    places_used_in: [
      '🇯🇴 Jordan - National',
      '🇰🇼 Kuwait - National',
      '🇱🇾 Libya - National',
      '🇴🇲 Oman - National',
      '🇶🇦 Qatar - National'
    ],
    election_type: ElectionType.SingleNonTransferable
  },
  {
    title: 'What is First Past the Post Vote?',
    summary:
      "First Past the Post (FPTP) is a voting method where the candidate with the most votes in each district wins. It's used in single-member district elections and is the simplest form of plurality/majority system.",
    benefits: [],
    issues: [
      'Does not reflect proportionality of votes',
      'May discourage voters from choosing third-party or independent candidates',
      'Winner takes all approach can lead to significant portions of the electorate feeling unrepresented'
    ],
    link: 'https://en.wikipedia.org/wiki/First-past-the-post_voting',
    places_used_in: [
      '🇺🇸 United States - Most states',
      '🇬🇧 United Kingdom - General',
      '🇨🇦 Canada - Federal',
      '🇮🇳 India - Parliamentary',
      '🇫🇷 France - Legislative (Some cases)',
      '🇩🇪 Germany - Part of the vote for Bundestag',
      '🇰🇷 South Korea - National Assembly',
      '🇯🇵 Japan - House of Representatives',
      '🇹🇼 Taiwan - Legislative Yuan',
      '🇧🇷 Brazil - Lower House',
      '🇷🇺 Russia - Duma (Some cases)',
      '🇲🇽 Mexico - Chamber of Deputies',
      '🇸🇦 Saudi Arabia - Municipal (Men only)',
      '🇲🇳 Mongolia - Parliamentary',
      '🇨🇴 Colombia - House of Representatives',
      '🇿🇦 South Africa - National Assembly (Partially)',
      "🇮🇩 Indonesia - People's Consultative Assembly (Partially)",
      '🇧🇩 Bangladesh - Jatiya Sangsad',
      '🇧🇪 Belgium - Federal Parliament',
      '🇧🇿 Belize - House of Representatives',
      '🇧🇸 Bahamas - House of Assembly',
      '🇧🇧 Barbados - House of Assembly',
      '🇧🇴 Bolivia - Plurinational Legislative Assembly',
      '🇧🇬 Bulgaria - National Assembly',
      '🇬🇲 Gambia - Presidential',
      '🇬🇭 Ghana - Presidential',
      '🇬🇩 Grenada - National',
      '🇬🇾 Guyana - National',
      '🇭🇳 Honduras - Presidential',
      '🇯🇲 Jamaica - National',
      '🇰🇪 Kenya - Presidential, Legislative',
      '🇰🇮 Kiribati - National',
      '🇱🇷 Liberia - Legislative',
      '🇲🇼 Malawi - (Presidential, Legislative)',
      '🇲🇾 Malaysia - National',
      '🇲🇻 Maldives - Parliamentary',
      '🇲🇭 Marshall Islands - (Presidential, Legislative)',
      '🇲🇺 Mauritius - National',
      '🇲🇽 Mexico - Presidential',
      '🇫🇲 Micronesia - National',
      '🇲🇨 Monaco - National',
      '🇲🇿 Mozambique - Presidential',
      '🇲🇲 Myanmar (Burma) - National',
      '🇳🇮 Nicaragua - Presidential',
      '🇳🇬 Nigeria - (Presidential, Legislative)',
      '🇵🇰 Pakistan - National',
      '🇵🇼 Palau - National',
      '🇵🇦 Panama - Presidential',
      '🇵🇾 Paraguay - Presidential',
      '🇵🇭 Philippines - Presidential',
      '🇰🇳 St Kitts & Nevis - National',
      '🇱🇨 St Lucia - National',
      '🇻🇨 Saint Vincent & the Grenadines - National',
      '🇼🇸 Samoa - National',
      '🇸🇲 San Marino - National',
      '🇸🇨 Seychelles - Legislative',
      '🇸🇬 Singapore - (Presidential, Legislative)'
    ],
    election_type: ElectionType.FirstPastThePost
  },
  {
    title: 'Quota Preferential Victoria Labor 2024?',
    summary:
      "This is the voting system used to elect vic Labor state delegates for the state conference. It's is a modified version of the single transferable vote with a affirmative action clause ensuring 50% of elected delegates area women. Only parts 1, 2, 3, 4, 5, 6, 7 are implemented.",
    benefits: [],
    issues: [],
    link: '/Victorian-Labor-Rules-Aug-2024.pdf',
    places_used_in: ['🇦🇺 Victoria Australia - Victorian Labor Party'],
    election_type: ElectionType.QuotaPreferentialVicLabor2024
  }
]

function get_info(): Info {
  const result = info.find((element) => {
    return element.election_type === props.election_type
  })

  if (result) return result

  return {
    title: 'Unknown election type no info please contact paul',
    summary: '',
    benefits: [],
    issues: [],
    link: 'sarda.dev',
    places_used_in: [],
    election_type: ElectionType.Approval
  }
}

const selected_info = ref<Info>(get_info())
</script>

<template>
  <div>
    <p class="header">{{ selected_info.title }}</p>
    <p>{{ selected_info.summary }}</p>
    <div v-if="selected_info.benefits.length > 0">
      <p class="second-header">Benefits</p>
      <ul>
        <li v-for="(benefit, i) in selected_info.benefits" :key="i">{{ benefit }}</li>
      </ul>
    </div>
    <div v-if="selected_info.issues.length > 0">
      <p class="second-header">Issues</p>
      <ul>
        <li v-for="(issue, i) in selected_info.issues" :key="i">{{ issue }}</li>
      </ul>
    </div>
    <p class="second-header">Countries Used in</p>
    <ul v-if="selected_info.places_used_in.length > 0">
      <li v-for="(county, i) in selected_info.places_used_in" :key="i">{{ county }}</li>
    </ul>
    <p v-else>Might be not sure 🤷</p>
    <p class="second-header"><a :href="selected_info.link" target="_blank">Read more</a></p>
  </div>
</template>

<style scoped>
.header {
  font-weight: 700;
}

.second-header {
  font-weight: 600;
}
</style>
