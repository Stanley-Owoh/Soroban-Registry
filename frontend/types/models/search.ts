import { Network } from "./network";
import { Contract } from "./contract";
import { PaginatedResponse } from "./common";

/**
 * Search related types
 */

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

export interface SemanticContractSearchResponse extends PaginatedResponse<Contract> {
  semantic: SemanticSearchMetadata;
}

<<<<<<< HEAD
/**
 * Advanced query types for contract search
 */

export type FieldOperator =
  | "eq"
  | "ne"
  | "gt"
  | "lt"
  | "in"
  | "contains"
  | "starts_with";

export interface QueryCondition {
  field: string;
  operator: FieldOperator;
  value: string | number | boolean | string[];
}

export type QueryOperator = "AND" | "OR";

export type QueryNode =
  | QueryCondition
  | { operator: QueryOperator; conditions: QueryNode[] };
=======
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
>>>>>>> main
