// Mock data: conditionally imported only in development/test.
// In production (NEXT_PUBLIC_USE_MOCKS !== "true"), these are empty stubs
// that never get reached (gated behind USE_MOCKS checks below).
/* eslint-disable @typescript-eslint/no-explicit-any */
let MOCK_CONTRACTS: any[] = [];
let MOCK_EXAMPLES: Record<string, any[]> = {};
let MOCK_VERSIONS: Record<string, any[]> = {};
/* eslint-enable @typescript-eslint/no-explicit-any */
if (process.env.NEXT_PUBLIC_USE_MOCKS === "true") {
  // Dynamic require ensures Next.js tree-shakes mock-data from production bundles
  // eslint-disable-next-line @typescript-eslint/no-require-imports
  const mocks = require("./mock-data");
  MOCK_CONTRACTS = mocks.MOCK_CONTRACTS;
  MOCK_EXAMPLES = mocks.MOCK_EXAMPLES;
  MOCK_VERSIONS = mocks.MOCK_VERSIONS;
}
import { CollaborativeComment, CollaborativeReviewDetails, VerificationLevel } from "@/types";
import { trackEvent } from "./analytics";
import {
  ApiError,
  NetworkError,
  extractErrorData,
  createApiError,
} from "./errors";

export type Network = "mainnet" | "testnet" | "futurenet";

export type NetworkStatus = "online" | "offline" | "degraded";

export interface NetworkEndpoints {
  rpc_url: string;
  health_url: string;
  explorer_url: string;
  friendbot_url?: string;
}

export interface NetworkInfo {
  id: string;
  name: string;
  network_type: Network;
  status: NetworkStatus;
  endpoints: NetworkEndpoints;
  last_checked_at: string;
  last_indexed_ledger_height?: number;
  last_indexed_at?: string;
  consecutive_failures: number;
  status_message?: string;
}

export interface NetworkListResponse {
  networks: NetworkInfo[];
  cached_at: string;
}

/** Per-network config (Issue #43) */
export interface NetworkConfig {
  contract_id: string;
  is_verified: boolean;
  min_version?: string;
  max_version?: string;
}

export interface Contract {
  id: string;
  contract_id: string;
  wasm_hash: string;
  name: string;
  description?: string;
  publisher_id: string;
  network: Network;
  is_verified: boolean;
  verification_level?: VerificationLevel;
  category?: string;
  tags: string[];
  popularity_score?: number;
  downloads?: number;
  average_rating?: number;
  avg_rating?: number;
  review_count?: number;
  deployment_count?: number;
  interaction_count?: number;
  relevance_score?: number;
  // Image fields for contract logo/icon
  logo_url?: string;
  created_at: string;
  updated_at: string;
  verified_at?: string;
  last_accessed_at?: string;
  is_maintenance?: boolean;
  /** Logical contract grouping (Issue #43) */
  logical_id?: string;
  /** Per-network configs: { mainnet: {...}, testnet: {...} } */
  network_configs?: Record<Network, NetworkConfig>;
}

/** GET /contracts/:id response when ?network= is used (Issue #43) */
export interface ContractGetResponse extends Contract {
  current_network?: Network;
  network_config?: NetworkConfig;
}

export interface ContractHealth {
  contract_id: string;
  status: "healthy" | "warning" | "critical";
  last_activity: string;
  security_score: number;
  audit_date?: string;
  total_score: number;
  recommendations: string[];
  updated_at: string;
}

export interface ContractInteractionResponse {
  id: string;
  account: string | null;
  method: string | null;
  parameters: unknown;
  return_value: unknown;
  transaction_hash: string | null;
  created_at: string;
}

export interface InteractionsQueryParams {
  limit?: number;
  offset?: number;
  account?: string;
  method?: string;
  from_timestamp?: string;
  to_timestamp?: string;
}

export interface InteractionsListResponse {
  items: ContractInteractionResponse[];
  total: number;
  limit: number;
  offset: number;
}

/** Analytics timeline entry (one day) */
export interface TimelineEntry {
  date: string;
  count: number;
}

export interface TopUser {
  address: string;
  count: number;
}

export interface InteractorStats {
  unique_count: number;
  top_users: TopUser[];
}

export interface DeploymentStats {
  count: number;
  unique_users: number;
  by_network: Record<string, number>;
}

export interface ContractAnalyticsResponse {
  contract_id: string;
  deployments: DeploymentStats;
  interactors: InteractorStats;
  timeline: TimelineEntry[];
}

export interface ContractVersion {
  id: string;
  contract_id: string;
  version: string;
  wasm_hash: string;
  source_url?: string;
  commit_hash?: string;
  release_notes?: string;
  created_at: string;
}

export interface ContractAbiResponse {
  abi: unknown;
}

export interface ContractChangelogEntry {
  version: string;
  created_at: string;
  commit_hash?: string;
  source_url?: string;
  release_notes?: string;
  breaking: boolean;
  breaking_changes: string[];
}

export interface ContractChangelogResponse {
  contract_id: string;
  entries: ContractChangelogEntry[];
}

export interface RecommendationReason {
  code: string;
  message: string;
  weight: number;
}

export interface RecommendedContract {
  id: string;
  contract_id: string;
  name: string;
  description?: string;
  network: Network;
  category?: string;
  popularity_score: number;
  similarity_score: number;
  recommendation_score: number;
  reasons: RecommendationReason[];
  explanation: string;
}

export interface ContractRecommendationsResponse {
  contract_id: string;
  algorithm: string;
  ab_variant: string;
  cached: boolean;
  generated_at: string;
  recommendations: RecommendedContract[];
}

export interface Publisher {
  id: string;
  stellar_address: string;
  username?: string;
  email?: string;
  github_url?: string;
  website?: string;
  // Image fields for publisher avatar
  avatar_url?: string;
  created_at: string;
}

export type AnalyticsEventType = 
  | 'contract_published' 
  | 'contract_verified' 
  | 'contract_deployed' 
  | 'version_created' 
  | 'contract_updated' 
  | 'publisher_created' 
  | 'search_click';

export interface AnalyticsEvent {
  id: string;
  event_type: AnalyticsEventType;
  contract_id: string;
  user_address: string | null;
  network: Network | null;
  metadata: Record<string, unknown> | null;
  created_at: string;
}

export interface ActivityFeedParams {
  cursor?: string;
  limit?: number;
  event_type?: AnalyticsEventType;
  contract_id?: string;
}

export interface ActivityFeedResponse {
  items: AnalyticsEvent[];
  total: number;
  limit: number;
  next_cursor: string | null;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

export interface DependencyTreeNode {
  contract_id: string;
  name: string;
  current_version: string;
  constraint_to_parent: string;
  dependencies: DependencyTreeNode[];
}

export interface MaintenanceWindow {
  message: string;
  scheduled_end_at?: string;
}

export type MaturityLevel = 'alpha' | 'beta' | 'stable' | 'mature' | 'legacy';

export interface ContractSearchParams {
  query?: string;
  contract_id?: string;
  network?: "mainnet" | "testnet" | "futurenet";
  networks?: Array<"mainnet" | "testnet" | "futurenet">;
  verified_only?: boolean;
  favorites_only?: boolean;
  favorites_list?: string[];
  category?: string;
  categories?: string[];
  language?: string;
  languages?: string[];
  author?: string;
  tags?: string[];
  maturity?: 'alpha' | 'beta' | 'stable' | 'mature' | 'legacy';
  page?: number;
  page_size?: number;
  sort_by?: 'name' | 'created_at' | 'updated_at' | 'popularity' | 'deployments' | 'interactions' | 'relevance' | 'downloads' | 'rating';
  sort_order?: 'asc' | 'desc';
  date_from?: string;
  date_to?: string;
}

export interface SearchSuggestion {
  text: string;
  kind: string;
  score: number;
}

export interface SearchSuggestionsResponse {
  items: SearchSuggestion[];
}

export type SearchIntentType =
  | "generic"
  | "category"
  | "network"
  | "verification"
  | "tag"
  | "author";

export interface SearchIntent {
  type: SearchIntentType;
  confidence: number;
  extracted: {
    categories: string[];
    tags: string[];
    networks: Network[];
    verified_only: boolean;
    author?: string;
  };
}

export interface SemanticSearchMetadata {
  raw_query: string;
  interpreted_query: string;
  intent: SearchIntent;
  fallback_used: boolean;
  query_suggestions: string[];
}

export interface SemanticContractSearchResponse
  extends PaginatedResponse<Contract> {
  semantic: SemanticSearchMetadata;
}

export interface PublishRequest {
  contract_id: string;
  name: string;
  description?: string;
  network: "mainnet" | "testnet" | "futurenet";
  category?: string;
  tags: string[];
  source_url?: string;
  publisher_address: string;
}

export type CustomMetricType = 'counter' | 'gauge' | 'histogram';

export interface MetricCatalogEntry {
  metric_name: string;
  metric_type: CustomMetricType;
  last_seen: string;
  sample_count: number;
}

export interface MetricSeriesPoint {
  bucket_start: string;
  bucket_end: string;
  sample_count: number;
  sum_value?: number;
  avg_value?: number;
  min_value?: number;
  max_value?: number;
  p50_value?: number;
  p95_value?: number;
  p99_value?: number;
}

export interface MetricSample {
  timestamp: string;
  value: number;
  unit?: string;
  metadata?: Record<string, unknown> | null;
}

export interface MetricSeriesResponse {
  contract_id: string;
  metric_name: string;
  metric_type: CustomMetricType | null;
  resolution: 'hour' | 'day' | 'raw';
  points?: MetricSeriesPoint[];
  samples?: MetricSample[];
}

export type DeprecationStatus = 'active' | 'deprecated' | 'retired';

export type ReleaseNotesStatus = 'draft' | 'published';

export interface FunctionChange {
  name: string;
  change_type: 'added' | 'removed' | 'modified';
  old_signature?: string;
  new_signature?: string;
  is_breaking: boolean;
}

export interface DiffSummary {
  files_changed: number;
  lines_added: number;
  lines_removed: number;
  function_changes: FunctionChange[];
  has_breaking_changes: boolean;
  features_count: number;
  fixes_count: number;
  breaking_count: number;
}

export interface ReleaseNotesResponse {
  id: string;
  contract_id: string;
  version: string;
  previous_version?: string;
  diff_summary: DiffSummary;
  changelog_entry?: string;
  notes_text: string;
  status: ReleaseNotesStatus;
  generated_by: string;
  created_at: string;
  updated_at: string;
  published_at?: string;
}

export interface GenerateReleaseNotesRequest {
  version: string;
  previous_version?: string;
  source_url?: string;
  changelog_content?: string;
  contract_address?: string;
}

export interface UpdateReleaseNotesRequest {
  notes_text: string;
}

export interface PublishReleaseNotesRequest {
  update_version_record?: boolean;
}

export interface DeprecationInfo {
  contract_id: string;
  status: DeprecationStatus;
  deprecated_at?: string | null;
  retirement_at?: string | null;
  replacement_contract_id?: string | null;
  migration_guide_url?: string | null;
  notes?: string | null;
  days_remaining?: number | null;
  dependents_notified: number;
}

const API_URL = process.env.NEXT_PUBLIC_API_URL || "";
const USE_MOCKS = process.env.NEXT_PUBLIC_USE_MOCKS === "true";

const CATEGORY_SYNONYMS: Record<string, string> = {
  defi: "DeFi",
  dex: "DeFi",
  lending: "DeFi",
  nft: "NFT",
  governance: "Governance",
  infra: "Infrastructure",
  infrastructure: "Infrastructure",
  payment: "Payment",
  payments: "Payment",
  identity: "Identity",
  game: "Gaming",
  gaming: "Gaming",
  social: "Social",
};

function tokenizeQuery(query: string): string[] {
  return query
    .toLowerCase()
    .replace(/[^\w\s]/g, " ")
    .split(/\s+/)
    .map((token) => token.trim())
    .filter(Boolean);
}

function dedupe<T>(values: T[]): T[] {
  return Array.from(new Set(values));
}

function detectIntent(query: string, params?: ContractSearchParams): SearchIntent {
  const tokens = tokenizeQuery(query);
  const categories = dedupe(
    tokens
      .map((token) => CATEGORY_SYNONYMS[token])
      .filter((value): value is string => Boolean(value)),
  );

  const networks = dedupe(
    tokens
      .map((token) => {
        if (token.includes("mainnet")) return "mainnet";
        if (token.includes("testnet")) return "testnet";
        if (token.includes("futurenet")) return "futurenet";
        return undefined;
      })
      .filter((value): value is Network => Boolean(value)),
  );

  const verifiedOnly =
    tokens.includes("verified") || tokens.includes("audited") || Boolean(params?.verified_only);

  const authorTokenIndex = tokens.findIndex(
    (token) => token === "by" || token === "from" || token === "author",
  );
  const author =
    params?.author ||
    (authorTokenIndex >= 0 && tokens[authorTokenIndex + 1]
      ? tokens[authorTokenIndex + 1]
      : undefined);

  let type: SearchIntentType = "generic";
  if (categories.length > 0) type = "category";
  else if (networks.length > 0) type = "network";
  else if (verifiedOnly) type = "verification";
  else if (author) type = "author";

  const confidence = Math.min(
    0.98,
    0.35 +
      (categories.length > 0 ? 0.2 : 0) +
      (networks.length > 0 ? 0.15 : 0) +
      (verifiedOnly ? 0.15 : 0) +
      (author ? 0.15 : 0),
  );

  return {
    type,
    confidence,
    extracted: {
      categories,
      tags: [],
      networks,
      verified_only: verifiedOnly,
      author,
    },
  };
}

function semanticScore(contract: Contract, queryTokens: string[], intent: SearchIntent): number {
  const haystack = [
    contract.name,
    contract.description || "",
