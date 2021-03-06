//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.3
//!

            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde::{{YaSerialize, YaDeserialize}};
            use std::io::{Read, Write};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Fault",
	namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
	prefix = "soapenv",
)]
pub struct SoapFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>, 
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>, 
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BeaFault",
)]
pub struct BeaFault {
	#[yaserde(rename = "BeaFault", default)]
	pub bea_fault: BeAFaultSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageRequest",
)]
pub struct GetMessageRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageResponse",
)]
pub struct GetMessageResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createSessionKeyRequest",
)]
pub struct CreateSessionKeyRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::CreateSessionKeyRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createSessionKeyResponse",
)]
pub struct CreateSessionKeyResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::CreateSessionKeyResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createNewMessageRequest",
)]
pub struct CreateNewMessageRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::CreateNewMessageRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createNewMessageResponse",
)]
pub struct CreateNewMessageResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::CreateNewMessageResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "saveMessageRequest",
)]
pub struct SaveMessageRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::SaveMessageRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "saveMessageResponse",
)]
pub struct SaveMessageResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::SaveMessageResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "sendMessageRequest",
)]
pub struct SendMessageRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::SendMessageRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "sendMessageResponse",
)]
pub struct SendMessageResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::SendMessageResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderStructureRequest",
)]
pub struct GetFolderStructureRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetFolderStructureRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderStructureResponse",
)]
pub struct GetFolderStructureResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetFolderStructureResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderOverviewRequest",
)]
pub struct GetFolderOverviewRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetFolderOverviewRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderOverviewResponse",
)]
pub struct GetFolderOverviewResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetFolderOverviewResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "searchAddresseeRequest",
)]
pub struct SearchAddresseeRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::SearchAddresseeRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "searchAddresseeResponse",
)]
pub struct SearchAddresseeResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::SearchAddresseeResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFavouriteAddresseeRequest",
)]
pub struct GetFavouriteAddresseeRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetFavouriteAddresseeRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFavouriteAddresseeResponse",
)]
pub struct GetFavouriteAddresseeResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetFavouriteAddresseeResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteFavouriteAddresseeRequest",
)]
pub struct DeleteFavouriteAddresseeRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteFavouriteAddresseeRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteFavouriteAddresseeResponse",
)]
pub struct DeleteFavouriteAddresseeResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteFavouriteAddresseeResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToFolderRequest",
)]
pub struct MoveMessageToFolderRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::MoveMessageToFolderRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToFolderResponse",
)]
pub struct MoveMessageToFolderResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::MoveMessageToFolderResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToTrashRequest",
)]
pub struct MoveMessageToTrashRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::MoveMessageToTrashRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToTrashResponse",
)]
pub struct MoveMessageToTrashResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::MoveMessageToTrashResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "restoreMessageFromTrashRequest",
)]
pub struct RestoreMessageFromTrashRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::RestoreMessageFromTrashRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "restoreMessageFromTrashResponse",
)]
pub struct RestoreMessageFromTrashResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::RestoreMessageFromTrashResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteMessageRequest",
)]
pub struct DeleteMessageRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteMessageRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteMessageResponse",
)]
pub struct DeleteMessageResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteMessageResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addFolderRequest",
)]
pub struct AddFolderRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::AddFolderRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addFolderResponse",
)]
pub struct AddFolderResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::AddFolderResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateFolderRequest",
)]
pub struct UpdateFolderRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::UpdateFolderRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateFolderResponse",
)]
pub struct UpdateFolderResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::UpdateFolderResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeFolderRequest",
)]
pub struct RemoveFolderRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::RemoveFolderRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeFolderResponse",
)]
pub struct RemoveFolderResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::RemoveFolderResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addCommentRequest",
)]
pub struct AddCommentRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::AddCommentRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addCommentResponse",
)]
pub struct AddCommentResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::AddCommentResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateCommentRequest",
)]
pub struct UpdateCommentRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::UpdateCommentRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateCommentResponse",
)]
pub struct UpdateCommentResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::UpdateCommentResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeCommentRequest",
)]
pub struct RemoveCommentRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::RemoveCommentRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeCommentResponse",
)]
pub struct RemoveCommentResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::RemoveCommentResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "loginUserRequest",
)]
pub struct LoginUserRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::LoginUserRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "loginUserResponse",
)]
pub struct LoginUserResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::LoginUserResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "logoutUserRequest",
)]
pub struct LogoutUserRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::LogoutUserRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "logoutUserResponse",
)]
pub struct LogoutUserResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::LogoutUserResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startLoginUserRequest",
)]
pub struct StartLoginUserRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::StartLoginUserRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startLoginUserResponse",
)]
pub struct StartLoginUserResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::StartLoginUserResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addUserRequest",
)]
pub struct AddUserRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::AddUserRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addUserResponse",
)]
pub struct AddUserResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::AddUserResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateUserRequest",
)]
pub struct UpdateUserRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::UpdateUserRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateUserResponse",
)]
pub struct UpdateUserResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::UpdateUserResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteUserRequest",
)]
pub struct DeleteUserRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteUserRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteUserResponse",
)]
pub struct DeleteUserResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteUserResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAccessRightRequest",
)]
pub struct AddAccessRightRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::AddAccessRightRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAccessRightResponse",
)]
pub struct AddAccessRightResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::AddAccessRightResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAccessRightRequest",
)]
pub struct DeleteAccessRightRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteAccessRightRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAccessRightResponse",
)]
pub struct DeleteAccessRightResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteAccessRightResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxJournalRequest",
)]
pub struct GetPostboxJournalRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetPostboxJournalRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxJournalResponse",
)]
pub struct GetPostboxJournalResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetPostboxJournalResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageJournalRequest",
)]
pub struct GetMessageJournalRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageJournalRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageJournalResponse",
)]
pub struct GetMessageJournalResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageJournalResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserNameRequest",
)]
pub struct GetUserNameRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetUserNameRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserNameResponse",
)]
pub struct GetUserNameResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetUserNameResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserJournalRequest",
)]
pub struct GetUserJournalRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetUserJournalRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserJournalResponse",
)]
pub struct GetUserJournalResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetUserJournalResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAuthentificationRequest",
)]
pub struct DeleteAuthentificationRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteAuthentificationRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAuthentificationResponse",
)]
pub struct DeleteAuthentificationResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteAuthentificationResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAuthentificationRequest",
)]
pub struct AddAuthentificationRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::AddAuthentificationRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAuthentificationResponse",
)]
pub struct AddAuthentificationResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::AddAuthentificationResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startRegistrationRequest",
)]
pub struct StartRegistrationRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::StartRegistrationRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startRegistrationResponse",
)]
pub struct StartRegistrationResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::StartRegistrationResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "finishRegistrationRequest",
)]
pub struct FinishRegistrationRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::FinishRegistrationRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "finishRegistrationResponse",
)]
pub struct FinishRegistrationResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::FinishRegistrationResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAddressbookRequest",
)]
pub struct GetAddressbookRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetAddressbookRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAddressbookResponse",
)]
pub struct GetAddressbookResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetAddressbookResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAddressbookEntryRequest",
)]
pub struct DeleteAddressbookEntryRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteAddressbookEntryRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAddressbookEntryResponse",
)]
pub struct DeleteAddressbookEntryResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::DeleteAddressbookEntryResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAddressbookEntryRequest",
)]
pub struct AddAddressbookEntryRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::AddAddressbookEntryRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAddressbookEntryResponse",
)]
pub struct AddAddressbookEntryResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::AddAddressbookEntryResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxesRequest",
)]
pub struct GetPostboxesRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetPostboxesRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxesResponse",
)]
pub struct GetPostboxesResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetPostboxesResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getConfigurationRequest",
)]
pub struct GetConfigurationRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetConfigurationRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getConfigurationResponse",
)]
pub struct GetConfigurationResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetConfigurationResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getRecipientDataRequest",
)]
pub struct GetRecipientDataRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetRecipientDataRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getRecipientDataResponse",
)]
pub struct GetRecipientDataResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetRecipientDataResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getIdentityDataRequest",
)]
pub struct GetIdentityDataRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetIdentityDataRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getIdentityDataResponse",
)]
pub struct GetIdentityDataResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetIdentityDataResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getProcesscardsRequest",
)]
pub struct GetProcesscardsRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetProcesscardsRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getProcesscardsResponse",
)]
pub struct GetProcesscardsResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetProcesscardsResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "setProcesscardsRequest",
)]
pub struct SetProcesscardsRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::SetProcesscardsRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "setProcesscardsResponse",
)]
pub struct SetProcesscardsResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::SetProcesscardsResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationResultRequest",
)]
pub struct GetVerificationResultRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetVerificationResultRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationResultResponse",
)]
pub struct GetVerificationResultResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetVerificationResultResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationConfigRequest",
)]
pub struct GetVerificationConfigRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetVerificationConfigRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationConfigResponse",
)]
pub struct GetVerificationConfigResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetVerificationConfigResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageConfigRequest",
)]
pub struct GetMessageConfigRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageConfigRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageConfigResponse",
)]
pub struct GetMessageConfigResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageConfigResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "activateAuthenticationRequest",
)]
pub struct ActivateAuthenticationRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::ActivateAuthenticationRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "activateAuthenticationResponse",
)]
pub struct ActivateAuthenticationResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::ActivateAuthenticationResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUnactivatedSecurityTokensRequest",
)]
pub struct GetUnactivatedSecurityTokensRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetUnactivatedSecurityTokensRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUnactivatedSecurityTokensResponse",
)]
pub struct GetUnactivatedSecurityTokensResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetUnactivatedSecurityTokensResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsRequest",
)]
pub struct GetAccessRightsRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetAccessRightsRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsResponse",
)]
pub struct GetAccessRightsResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetAccessRightsResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsGrantedRequest",
)]
pub struct GetAccessRightsGrantedRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetAccessRightsGrantedRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsGrantedResponse",
)]
pub struct GetAccessRightsGrantedResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetAccessRightsGrantedResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageOverviewRequest",
)]
pub struct GetMessageOverviewRequest {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageOverviewRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageOverviewResponse",
)]
pub struct GetMessageOverviewResponse {
	#[yaserde(flatten, default)]
	pub parameters: types::GetMessageOverviewResponse, 
}
}

pub mod types {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BeAFaultSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct BeAFaultSoapDTO {
	#[yaserde(rename = "faultId", prefix = "nsi2", default)]
	pub fault_id: i64, 
	#[yaserde(rename = "faultCode", prefix = "nsi2", default)]
	pub fault_code: String, 
	#[yaserde(rename = "faultMessage", prefix = "nsi2", default)]
	pub fault_message: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MessageSoapDTO {
	#[yaserde(rename = "osciMessageId", prefix = "nsi2", default)]
	pub osci_message_id: Option<String>, 
	#[yaserde(rename = "osciSubject", prefix = "nsi2", default)]
	pub osci_subject: OscisubjectTypeSoapDTO, 
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: i64, 
	#[yaserde(rename = "attachments", prefix = "nsi2", default)]
	pub attachments: Vec<AttachmentSoapDTO>, 
	#[yaserde(rename = "encryptedObject", prefix = "nsi2", default)]
	pub encrypted_object: Vec<EncryptedObjectSoapDTO>, 
	#[yaserde(rename = "metaData", prefix = "nsi2", default)]
	pub meta_data: MetaDataSoapDTO, 
	#[yaserde(rename = "newEGVPMessage", prefix = "nsi2", default)]
	pub new_egvp_message: bool, 
	#[yaserde(rename = "certificateMap", prefix = "nsi2", default)]
	pub certificate_map: Option<CertificateMapSoapDTO>, 
	#[yaserde(rename = "version", prefix = "nsi2", default)]
	pub version: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SecurityTokenTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SecurityTokenTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SecurityTokenSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SecurityTokenSoapDTO {
	#[yaserde(rename = "tokenId", prefix = "nsi2", default)]
	pub token_id: Option<i64>, 
	#[yaserde(rename = "tokenName", prefix = "nsi2", default)]
	pub token_name: String, 
	#[yaserde(rename = "tokenType", prefix = "nsi2", default)]
	pub token_type: SecurityTokenTypeSoapDTO, 
	#[yaserde(rename = "authCertificate", prefix = "nsi2", default)]
	pub auth_certificate: String, 
	#[yaserde(rename = "cryptoCertificate", prefix = "nsi2", default)]
	pub crypto_certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SignedPrivilegeTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SignedPrivilegeTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SignedPrivilegeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SignedPrivilegeSoapDTO {
	#[yaserde(rename = "privilegeType", prefix = "nsi2", default)]
	pub privilege_type: SignedPrivilegeTypeSoapDTO, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi2", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "signature", prefix = "nsi2", default)]
	pub signature: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UsernamePasswordSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct UsernamePasswordSoapDTO {
	#[yaserde(rename = "username", prefix = "nsi2", default)]
	pub username: String, 
	#[yaserde(rename = "password", prefix = "nsi2", default)]
	pub password: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SAMLResponseSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SamlresponseSoapDTO {
	#[yaserde(rename = "sessionId", prefix = "nsi2", default)]
	pub session_id: Option<String>, 
	#[yaserde(rename = "samlResponse", prefix = "nsi2", default)]
	pub saml_response: String, 
	#[yaserde(rename = "authCertificate", prefix = "nsi2", default)]
	pub auth_certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CertificateMapSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct CertificateMapSoapDTO {
	#[yaserde(rename = "certificateEntry", prefix = "nsi2", default)]
	pub certificate_entry: Vec<CertificateEntrySoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CertificateEntrySoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct CertificateEntrySoapDTO {
	#[yaserde(rename = "reference", prefix = "nsi2", default)]
	pub reference: String, 
	#[yaserde(rename = "certificate", prefix = "nsi2", default)]
	pub certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OSCISubjectTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct OscisubjectTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct AttachmentSoapDTO {
	#[yaserde(rename = "reference", prefix = "nsi2", default)]
	pub reference: String, 
	#[yaserde(rename = "alias", prefix = "nsi2", default)]
	pub alias: Option<String>, 
	#[yaserde(rename = "data", prefix = "nsi2", default)]
	pub data: String, 
	#[yaserde(rename = "key", prefix = "nsi2", default)]
	pub key: String, 
	#[yaserde(rename = "type", prefix = "nsi2", default)]
	pub rs_type: AttachmentTypeSoapDTO, 
	#[yaserde(rename = "sizeKB", prefix = "nsi2", default)]
	pub size_kb: i64, 
	#[yaserde(rename = "sizeEncryptedKB", prefix = "nsi2", default)]
	pub size_encrypted_kb: Option<i64>, 
	#[yaserde(rename = "hashValue", prefix = "nsi2", default)]
	pub hash_value: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct AttachmentTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EncryptedObjectSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct EncryptedObjectSoapDTO {
	#[yaserde(rename = "enc_data", prefix = "nsi2", default)]
	pub enc_data: String, 
	#[yaserde(rename = "enc_name", prefix = "nsi2", default)]
	pub enc_name: String, 
	#[yaserde(rename = "encKeyInfo", prefix = "nsi2", default)]
	pub enc_key_info: Vec<EncKeyInfoSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EncKeyInfoSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct EncKeyInfoSoapDTO {
	#[yaserde(rename = "encKey", prefix = "nsi2", default)]
	pub enc_key: EncryptedDataSoapDTO, 
	#[yaserde(rename = "encCertificate", prefix = "nsi2", default)]
	pub enc_certificate: String, 
	#[yaserde(rename = "safeId", prefix = "nsi2", default)]
	pub safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EncryptedDataSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct EncryptedDataSoapDTO {
	#[yaserde(rename = "iv", prefix = "nsi2", default)]
	pub iv: String, 
	#[yaserde(rename = "value", prefix = "nsi2", default)]
	pub value: String, 
	#[yaserde(rename = "tag", prefix = "nsi2", default)]
	pub tag: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageInfoSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MessageInfoSoapDTO {
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: i64, 
	#[yaserde(rename = "sender", prefix = "nsi2", default)]
	pub sender: IdentitySoapDTO, 
	#[yaserde(rename = "referenceNumber", prefix = "nsi2", default)]
	pub reference_number: Option<String>, 
	#[yaserde(rename = "referenceNumberOpposition", prefix = "nsi2", default)]
	pub reference_number_opposition: Option<String>, 
	#[yaserde(rename = "referenceNumberJustice", prefix = "nsi2", default)]
	pub reference_number_justice: Option<String>, 
	#[yaserde(rename = "created", prefix = "nsi2", default)]
	pub created: String, 
	#[yaserde(rename = "sent", prefix = "nsi2", default)]
	pub sent: Option<String>, 
	#[yaserde(rename = "received", prefix = "nsi2", default)]
	pub received: Option<String>, 
	#[yaserde(rename = "appointment", prefix = "nsi2", default)]
	pub appointment: Option<String>, 
	#[yaserde(rename = "urgent", prefix = "nsi2", default)]
	pub urgent: bool, 
	#[yaserde(rename = "checkRequired", prefix = "nsi2", default)]
	pub check_required: bool, 
	#[yaserde(rename = "confidential", prefix = "nsi2", default)]
	pub confidential: bool, 
	#[yaserde(rename = "folderId", prefix = "nsi2", default)]
	pub folder_id: i64, 
	#[yaserde(rename = "addressees", prefix = "nsi2", default)]
	pub addressees: Option<IdentityListSoapDTO>, 
	#[yaserde(rename = "attributes", prefix = "nsi2", default)]
	pub attributes: Option<AttributeSoapDTO>, 
	#[yaserde(rename = "comments", prefix = "nsi2", default)]
	pub comments: Vec<CommentSoapDTO>, 
	#[yaserde(rename = "labels", prefix = "nsi2", default)]
	pub labels: Vec<LabelSoapDTO>, 
	#[yaserde(rename = "attachments", prefix = "nsi2", default)]
	pub attachments: Vec<AttachmentSoapDTO>, 
	#[yaserde(rename = "readFlags", prefix = "nsi2", default)]
	pub read_flags: Option<IdentityListSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecipientSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct RecipientSoapDTO {
	#[yaserde(rename = "safeId", prefix = "nsi2", default)]
	pub safe_id: String, 
	#[yaserde(rename = "name", prefix = "nsi2", default)]
	pub name: String, 
	#[yaserde(rename = "certificate", prefix = "nsi2", default)]
	pub certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MetaDataSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MetaDataSoapDTO {
	#[yaserde(rename = "receptionTime", prefix = "nsi2", default)]
	pub reception_time: Option<String>, 
	#[yaserde(rename = "sender", prefix = "nsi2", default)]
	pub sender: Option<RecipientSoapDTO>, 
	#[yaserde(rename = "addressee", prefix = "nsi2", default)]
	pub addressee: Vec<RecipientSoapDTO>, 
	#[yaserde(rename = "subject", prefix = "nsi2", default)]
	pub subject: Option<EncryptedDataSoapDTO>, 
	#[yaserde(rename = "referenceNumber", prefix = "nsi2", default)]
	pub reference_number: Option<String>, 
	#[yaserde(rename = "referenceNumberOpposition", prefix = "nsi2", default)]
	pub reference_number_opposition: Option<String>, 
	#[yaserde(rename = "referenceJustice", prefix = "nsi2", default)]
	pub reference_justice: Option<String>, 
	#[yaserde(rename = "messageSigned", prefix = "nsi2", default)]
	pub message_signed: bool, 
	#[yaserde(rename = "oneAttachmentSigned", prefix = "nsi2", default)]
	pub one_attachment_signed: bool, 
	#[yaserde(rename = "urgent", prefix = "nsi2", default)]
	pub urgent: bool, 
	#[yaserde(rename = "checkRequired", prefix = "nsi2", default)]
	pub check_required: bool, 
	#[yaserde(rename = "confidential", prefix = "nsi2", default)]
	pub confidential: bool, 
	#[yaserde(rename = "appointment", prefix = "nsi2", default)]
	pub appointment: Option<String>, 
	#[yaserde(rename = "originatorCertificate", prefix = "nsi2", default)]
	pub originator_certificate: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SessionKeySoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SessionKeySoapDTO {
	#[yaserde(rename = "sessionKey", prefix = "nsi2", default)]
	pub session_key: String, 
	#[yaserde(rename = "encrypteddata", prefix = "nsi2", default)]
	pub encrypteddata: EncryptedDataSoapDTO, 
	#[yaserde(rename = "safeId", prefix = "nsi2", default)]
	pub safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PostboxesSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct PostboxesSoapDTO {
	#[yaserde(rename = "postbox", prefix = "nsi2", default)]
	pub postbox: Vec<PostboxSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PostboxSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct PostboxSoapDTO {
	#[yaserde(rename = "postboxSafeId", prefix = "nsi2", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "folder", prefix = "nsi2", default)]
	pub folder: Vec<FolderSoapDTO>, 
	#[yaserde(rename = "label", prefix = "nsi2", default)]
	pub label: Vec<LabelSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct FolderSoapDTO {
	#[yaserde(rename = "id", prefix = "nsi2", default)]
	pub id: Option<i64>, 
	#[yaserde(rename = "type", prefix = "nsi2", default)]
	pub rs_type: FolderTypeSoapDTO, 
	#[yaserde(rename = "name", prefix = "nsi2", default)]
	pub name: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi2", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "parentFolderId", prefix = "nsi2", default)]
	pub parent_folder_id: Option<i64>, 
	#[yaserde(rename = "childFolder", prefix = "nsi2", default)]
	pub child_folder: Vec<FolderSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LabelSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct LabelSoapDTO {
	#[yaserde(rename = "id", prefix = "nsi2", default)]
	pub id: i64, 
	#[yaserde(rename = "name", prefix = "nsi2", default)]
	pub name: String, 
	#[yaserde(rename = "color", prefix = "nsi2", default)]
	pub color: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi2", default)]
	pub postbox_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct FolderTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderOverviewSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct FolderOverviewSoapDTO {
	#[yaserde(rename = "messageOverview", prefix = "nsi2", default)]
	pub message_overview: Vec<MessageOverviewSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageOverviewSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MessageOverviewSoapDTO {
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: i64, 
	#[yaserde(rename = "encSubject", prefix = "nsi2", default)]
	pub enc_subject: EncryptedDataSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchAddresseeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct SearchAddresseeSoapDTO {
	#[yaserde(rename = "identitySafeId", prefix = "nsi2", default)]
	pub identity_safe_id: Option<String>, 
	#[yaserde(rename = "identityStatus ", prefix = "nsi2", default)]
	pub identity_status: Vec<IdentityStatusSoapDTO>, 
	#[yaserde(rename = "identityType", prefix = "nsi2", default)]
	pub identity_type: Vec<IdentityTypeSoapDTO>, 
	#[yaserde(rename = "identityUsername", prefix = "nsi2", default)]
	pub identity_username: Option<String>, 
	#[yaserde(rename = "identityFirstname", prefix = "nsi2", default)]
	pub identity_firstname: Option<String>, 
	#[yaserde(rename = "identitySurname", prefix = "nsi2", default)]
	pub identity_surname: Option<String>, 
	#[yaserde(rename = "identityPostalcode", prefix = "nsi2", default)]
	pub identity_postalcode: Option<String>, 
	#[yaserde(rename = "identityCity", prefix = "nsi2", default)]
	pub identity_city: Option<String>, 
	#[yaserde(rename = "identityChamberType", prefix = "nsi2", default)]
	pub identity_chamber_type: Vec<ChamberTypeSoapDTO>, 
	#[yaserde(rename = "identityChamberMembershipId", prefix = "nsi2", default)]
	pub identity_chamber_membership_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IdentityListSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct IdentityListSoapDTO {
	#[yaserde(rename = "addressee", prefix = "nsi2", default)]
	pub addressee: Vec<IdentitySoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IdentitySoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct IdentitySoapDTO {
	#[yaserde(rename = "identityId", prefix = "nsi2", default)]
	pub identity_id: Option<i64>, 
	#[yaserde(rename = "safeId", prefix = "nsi2", default)]
	pub safe_id: String, 
	#[yaserde(rename = "status", prefix = "nsi2", default)]
	pub status: IdentityStatusSoapDTO, 
	#[yaserde(rename = "type", prefix = "nsi2", default)]
	pub rs_type: IdentityTypeSoapDTO, 
	#[yaserde(rename = "formOfAdress", prefix = "nsi2", default)]
	pub form_of_adress: Option<String>, 
	#[yaserde(rename = "title", prefix = "nsi2", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "username", prefix = "nsi2", default)]
	pub username: String, 
	#[yaserde(rename = "firstname", prefix = "nsi2", default)]
	pub firstname: Option<String>, 
	#[yaserde(rename = "surname", prefix = "nsi2", default)]
	pub surname: String, 
	#[yaserde(rename = "street", prefix = "nsi2", default)]
	pub street: Option<String>, 
	#[yaserde(rename = "streetnumber", prefix = "nsi2", default)]
	pub streetnumber: Option<String>, 
	#[yaserde(rename = "postalcode", prefix = "nsi2", default)]
	pub postalcode: Option<String>, 
	#[yaserde(rename = "city", prefix = "nsi2", default)]
	pub city: Option<String>, 
	#[yaserde(rename = "federalState", prefix = "nsi2", default)]
	pub federal_state: Option<String>, 
	#[yaserde(rename = "country", prefix = "nsi2", default)]
	pub country: Option<String>, 
	#[yaserde(rename = "email", prefix = "nsi2", default)]
	pub email: Option<String>, 
	#[yaserde(rename = "phone", prefix = "nsi2", default)]
	pub phone: Option<String>, 
	#[yaserde(rename = "mobile", prefix = "nsi2", default)]
	pub mobile: Option<String>, 
	#[yaserde(rename = "fax", prefix = "nsi2", default)]
	pub fax: Option<String>, 
	#[yaserde(rename = "organization", prefix = "nsi2", default)]
	pub organization: Option<String>, 
	#[yaserde(rename = "organizationExtension", prefix = "nsi2", default)]
	pub organization_extension: Option<String>, 
	#[yaserde(rename = "chamber", prefix = "nsi2", default)]
	pub chamber: Option<ChamberTypeSoapDTO>, 
	#[yaserde(rename = "chamberMembershipId", prefix = "nsi2", default)]
	pub chamber_membership_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IdentityStatusSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct IdentityStatusSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IdentityTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct IdentityTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GenderSoapTypeDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct GenderSoapTypeDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ChamberTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct ChamberTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CommentSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct CommentSoapDTO {
	#[yaserde(rename = "id", prefix = "nsi2", default)]
	pub id: Option<i64>, 
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: i64, 
	#[yaserde(rename = "creatorIdentitySafeId", prefix = "nsi2", default)]
	pub creator_identity_safe_id: String, 
	#[yaserde(rename = "creationDate", prefix = "nsi2", default)]
	pub creation_date: String, 
	#[yaserde(rename = "comment", prefix = "nsi2", default)]
	pub comment: String, 
	#[yaserde(rename = "done", prefix = "nsi2", default)]
	pub done: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AuthentificationTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct AuthentificationTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttributeListSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct AttributeListSoapDTO {
	#[yaserde(rename = "attribute", prefix = "nsi2", default)]
	pub attribute: Vec<AttributeSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttributeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct AttributeSoapDTO {
	#[yaserde(rename = "checkRequired", prefix = "nsi2", default)]
	pub check_required: bool, 
	#[yaserde(rename = "urgent", prefix = "nsi2", default)]
	pub urgent: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrivilegeTypeSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct PrivilegeTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrivilegeListSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct PrivilegeListSoapDTO {
	#[yaserde(rename = "privilege", prefix = "nsi2", default)]
	pub privilege: Vec<PrivilegeTypeSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "JournalListSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct JournalListSoapDTO {
	#[yaserde(rename = "journals", prefix = "nsi2", default)]
	pub journals: Vec<JournalSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "JournalSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct JournalSoapDTO {
	#[yaserde(rename = "journalType", prefix = "nsi2", default)]
	pub journal_type: String, 
	#[yaserde(rename = "eventType", prefix = "nsi2", default)]
	pub event_type: String, 
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: Option<i64>, 
	#[yaserde(rename = "attachmentReference", prefix = "nsi2", default)]
	pub attachment_reference: Option<String>, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi2", default)]
	pub postbox_safe_id: Option<String>, 
	#[yaserde(rename = "sourceIdentity", prefix = "nsi2", default)]
	pub source_identity: IdentitySoapDTO, 
	#[yaserde(rename = "targetIdentity", prefix = "nsi2", default)]
	pub target_identity: IdentitySoapDTO, 
	#[yaserde(rename = "referenceIdentity", prefix = "nsi2", default)]
	pub reference_identity: Option<IdentitySoapDTO>, 
	#[yaserde(rename = "timestamp", prefix = "nsi2", default)]
	pub timestamp: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AuthentConfigurationSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct AuthentConfigurationSoapDTO {
	#[yaserde(rename = "address", prefix = "nsi2", default)]
	pub address: String, 
	#[yaserde(rename = "provider", prefix = "nsi2", default)]
	pub provider: String, 
	#[yaserde(rename = "sigAlgorithm", prefix = "nsi2", default)]
	pub sig_algorithm: String, 
	#[yaserde(rename = "receiverServlet", prefix = "nsi2", default)]
	pub receiver_servlet: String, 
	#[yaserde(rename = "untrustedCertificate", prefix = "nsi2", default)]
	pub untrusted_certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PeriodSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct PeriodSoapDTO {
	#[yaserde(rename = "from", prefix = "nsi2", default)]
	pub from: Option<String>, 
	#[yaserde(rename = "to", prefix = "nsi2", default)]
	pub to: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageListSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MessageListSoapDTO {
	#[yaserde(rename = "messagesSoapDTO", prefix = "nsi2", default)]
	pub messages_soap_dto: Vec<MessagesSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessagesSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MessagesSoapDTO {
	#[yaserde(rename = "message", prefix = "nsi2", default)]
	pub message: MessageSoapDTO, 
	#[yaserde(rename = "messageInfo", prefix = "nsi2", default)]
	pub message_info: MessageInfoSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProcessCardSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct ProcessCardSoapDTO {
	#[yaserde(rename = "processCardId", prefix = "nsi2", default)]
	pub process_card_id: i64, 
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: i64, 
	#[yaserde(rename = "conversationId", prefix = "nsi2", default)]
	pub conversation_id: Option<String>, 
	#[yaserde(rename = "type", prefix = "nsi2", default)]
	pub rs_type: Option<i64>, 
	#[yaserde(rename = "challange", prefix = "nsi2", default)]
	pub challange: Option<String>, 
	#[yaserde(rename = "data", prefix = "nsi2", default)]
	pub data: String, 
	#[yaserde(rename = "encKey", prefix = "nsi2", default)]
	pub enc_key: EncryptedDataSoapDTO, 
	#[yaserde(rename = "processCard", prefix = "nsi2", default)]
	pub process_card: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProcessCardsSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct ProcessCardsSoapDTO {
	#[yaserde(rename = "processcards", prefix = "nsi2", default)]
	pub processcards: Vec<ProcessCardSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "VerificationResultStatusSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct VerificationResultStatusSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "VerificationResultSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct VerificationResultSoapDTO {
	#[yaserde(rename = "messageId", prefix = "nsi2", default)]
	pub message_id: i64, 
	#[yaserde(rename = "html", prefix = "nsi2", default)]
	pub html: String, 
	#[yaserde(rename = "xml", prefix = "nsi2", default)]
	pub xml: String, 
	#[yaserde(rename = "status", prefix = "nsi2", default)]
	pub status: VerificationResultStatusSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "VerificationConfigSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct VerificationConfigSoapDTO {
	#[yaserde(rename = "serverURL", prefix = "nsi2", default)]
	pub server_url: String, 
	#[yaserde(rename = "serverSignatureCertificate", prefix = "nsi2", default)]
	pub server_signature_certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageConfigurationSoapDTO",
	namespace = "nsi2: http://brak.de/bea/application/dto/soap/dto1",
	prefix = "nsi2",
)]
pub struct MessageConfigurationSoapDTO {
	#[yaserde(rename = "maxAttachmentSizeInKB", prefix = "nsi2", default)]
	pub max_attachment_size_in_kb: i64, 
	#[yaserde(rename = "maxAttachmentCount", prefix = "nsi2", default)]
	pub max_attachment_count: i32, 
	#[yaserde(rename = "attachmentExtensionBlacklist", prefix = "nsi2", default)]
	pub attachment_extension_blacklist: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IdentityRaTypeSoapDTO",
	namespace = "nsi3: http://brak.de/bea/application/dto/soap/dto2",
	prefix = "nsi3",
)]
pub struct IdentityRaTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageStructureTypeSoapDTO",
	namespace = "nsi3: http://brak.de/bea/application/dto/soap/dto2",
	prefix = "nsi3",
)]
pub struct MessageStructureTypeSoapDTO {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AccessRightListSoapDTO",
	namespace = "nsi17: http://brak.de/bea/application/dto/soap/dto5",
	prefix = "nsi17",
)]
pub struct AccessRightListSoapDTO {
	#[yaserde(rename = "accessRightsSoapDTO", prefix = "nsi17", default)]
	pub access_rights_soap_dto: Vec<AccessRightDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AccessRightDTO",
	namespace = "nsi17: http://brak.de/bea/application/dto/soap/dto5",
	prefix = "nsi17",
)]
pub struct AccessRightDTO {
	#[yaserde(rename = "postboxSafeId", prefix = "nsi17", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "identitySafeId", prefix = "nsi17", default)]
	pub identity_safe_id: String, 
	#[yaserde(rename = "privilege", prefix = "nsi17", default)]
	pub privilege: PrivilegeTypeSoapDTO, 
	#[yaserde(rename = "from", prefix = "nsi17", default)]
	pub from: Option<String>, 
	#[yaserde(rename = "until", prefix = "nsi17", default)]
	pub until: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createSessionKeyRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct CreateSessionKeyRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createSessionKeyResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct CreateSessionKeyResponse {
	#[yaserde(rename = "sessionKey", prefix = "nsi1", default)]
	pub session_key: Option<SessionKeySoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAddressbookRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetAddressbookRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAddressbookResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetAddressbookResponse {
	#[yaserde(rename = "addressbook", prefix = "nsi1", default)]
	pub addressbook: IdentityListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAddressbookEntryRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteAddressbookEntryRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "addressbookEntrySafeId", prefix = "nsi1", default)]
	pub addressbook_entry_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAddressbookEntryResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteAddressbookEntryResponse {
	#[yaserde(rename = "addressbookEntryDeleted", prefix = "nsi1", default)]
	pub addressbook_entry_deleted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAddressbookEntryRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddAddressbookEntryRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "identitySafeId", prefix = "nsi1", default)]
	pub identity_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAddressbookEntryResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddAddressbookEntryResponse {
	#[yaserde(rename = "addressbookEntryAdded", prefix = "nsi1", default)]
	pub addressbook_entry_added: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
	#[yaserde(rename = "markAsExport", prefix = "nsi1", default)]
	pub mark_as_export: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageResponse {
	#[yaserde(rename = "message", prefix = "nsi1", default)]
	pub message: MessageSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createNewMessageRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct CreateNewMessageRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "createNewMessageResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct CreateNewMessageResponse {
	#[yaserde(rename = "message", prefix = "nsi1", default)]
	pub message: Option<MessageSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "loginUserRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct LoginUserRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "samlResponse", prefix = "nsi1", default)]
	pub saml_response: String, 
	#[yaserde(rename = "authCertificate", prefix = "nsi1", default)]
	pub auth_certificate: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "loginUserResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct LoginUserResponse {
	#[yaserde(rename = "sessionKey", prefix = "nsi1", default)]
	pub session_key: Option<SessionKeySoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "logoutUserRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct LogoutUserRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "logoutUserResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct LogoutUserResponse {
	#[yaserde(rename = "logoutSuccessful", prefix = "nsi1", default)]
	pub logout_successful: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startLoginUserRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct StartLoginUserRequest {
	#[yaserde(rename = "authtificationType", prefix = "nsi1", default)]
	pub authtification_type: AuthentificationTypeSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startLoginUserResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct StartLoginUserResponse {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "samlRequest", prefix = "nsi1", default)]
	pub saml_request: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "finishRegistrationRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct FinishRegistrationRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "securityToken", prefix = "nsi1", default)]
	pub security_token: SecurityTokenSoapDTO, 
	#[yaserde(rename = "signedPrivilege", prefix = "nsi1", default)]
	pub signed_privilege: Option<SignedPrivilegeSoapDTO>, 
	#[yaserde(rename = "securityQuestion", prefix = "nsi1", default)]
	pub security_question: String, 
	#[yaserde(rename = "securityAnswer", prefix = "nsi1", default)]
	pub security_answer: String, 
	#[yaserde(rename = "email", prefix = "nsi1", default)]
	pub email: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "finishRegistrationResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct FinishRegistrationResponse {
	#[yaserde(rename = "finishRegistrationCompleted", prefix = "nsi1", default)]
	pub finish_registration_completed: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startRegistrationRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct StartRegistrationRequest {
	#[yaserde(rename = "usernamePassword", prefix = "nsi1", default)]
	pub username_password: UsernamePasswordSoapDTO, 
	#[yaserde(rename = "samlResponse", prefix = "nsi1", default)]
	pub saml_response: SamlresponseSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "startRegistrationResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct StartRegistrationResponse {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "securityQuestionList", prefix = "nsi1", default)]
	pub security_question_list: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAuthentificationRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddAuthentificationRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "securityToken", prefix = "nsi1", default)]
	pub security_token: SecurityTokenSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAuthentificationResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddAuthentificationResponse {
	#[yaserde(rename = "authentificationAdded", prefix = "nsi1", default)]
	pub authentification_added: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAuthentificationRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteAuthentificationRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "securityTokenName", prefix = "nsi1", default)]
	pub security_token_name: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAuthentificationResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteAuthentificationResponse {
	#[yaserde(rename = "authentificationDeleted", prefix = "nsi1", default)]
	pub authentification_deleted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserJournalResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetUserJournalResponse {
	#[yaserde(rename = "userJournals", prefix = "nsi1", default)]
	pub user_journals: JournalListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserJournalRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetUserJournalRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "identitySafeId", prefix = "nsi1", default)]
	pub identity_safe_id: String, 
	#[yaserde(rename = "from", prefix = "nsi1", default)]
	pub from: Option<String>, 
	#[yaserde(rename = "until", prefix = "nsi1", default)]
	pub until: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserNameRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetUserNameRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "identitySafeId", prefix = "nsi1", default)]
	pub identity_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUserNameResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetUserNameResponse {
	#[yaserde(rename = "username", prefix = "nsi1", default)]
	pub username: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addUserRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddUserRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "gender", prefix = "nsi1", default)]
	pub gender: GenderSoapTypeDTO, 
	#[yaserde(rename = "title", prefix = "nsi1", default)]
	pub title: String, 
	#[yaserde(rename = "firstname", prefix = "nsi1", default)]
	pub firstname: String, 
	#[yaserde(rename = "surname", prefix = "nsi1", default)]
	pub surname: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addUserResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddUserResponse {
	#[yaserde(rename = "newIdentity", prefix = "nsi1", default)]
	pub new_identity: IdentitySoapDTO, 
	#[yaserde(rename = "username", prefix = "nsi1", default)]
	pub username: String, 
	#[yaserde(rename = "password", prefix = "nsi1", default)]
	pub password: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateUserRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct UpdateUserRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "identitySafeId", prefix = "nsi1", default)]
	pub identity_safe_id: String, 
	#[yaserde(rename = "gender", prefix = "nsi1", default)]
	pub gender: GenderSoapTypeDTO, 
	#[yaserde(rename = "title", prefix = "nsi1", default)]
	pub title: String, 
	#[yaserde(rename = "firstname", prefix = "nsi1", default)]
	pub firstname: String, 
	#[yaserde(rename = "surname", prefix = "nsi1", default)]
	pub surname: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateUserResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct UpdateUserResponse {
	#[yaserde(rename = "userUpdated", prefix = "nsi1", default)]
	pub user_updated: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteUserRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteUserRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "recieverIdentitySafeId", prefix = "nsi1", default)]
	pub reciever_identity_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteUserResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteUserResponse {
	#[yaserde(rename = "userDeleted", prefix = "nsi1", default)]
	pub user_deleted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAccessRightRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddAccessRightRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "reciverIdentitySafeId", prefix = "nsi1", default)]
	pub reciver_identity_safe_id: String, 
	#[yaserde(rename = "privilege", prefix = "nsi1", default)]
	pub privilege: Vec<PrivilegeTypeSoapDTO>, 
	#[yaserde(rename = "from", prefix = "nsi1", default)]
	pub from: Option<String>, 
	#[yaserde(rename = "until", prefix = "nsi1", default)]
	pub until: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addAccessRightResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddAccessRightResponse {
	#[yaserde(rename = "accessRightAdded", prefix = "nsi1", default)]
	pub access_right_added: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "saveMessageRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SaveMessageRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "message", prefix = "nsi1", default)]
	pub message: MessageSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "saveMessageResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SaveMessageResponse {
	#[yaserde(rename = "messageSaved", prefix = "nsi1", default)]
	pub message_saved: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "sendMessageRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SendMessageRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "message", prefix = "nsi1", default)]
	pub message: MessageSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "sendMessageResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SendMessageResponse {
	#[yaserde(rename = "messageSent", prefix = "nsi1", default)]
	pub message_sent: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderStructureRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetFolderStructureRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderStructureResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetFolderStructureResponse {
	#[yaserde(rename = "postbox", prefix = "nsi1", default)]
	pub postbox: PostboxSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getConfigurationRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetConfigurationRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getConfigurationResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetConfigurationResponse {
	#[yaserde(rename = "AuthentConfiguration", prefix = "nsi1", default)]
	pub authent_configuration: AuthentConfigurationSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxesRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetPostboxesRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxesResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetPostboxesResponse {
	#[yaserde(rename = "postboxes", prefix = "nsi1", default)]
	pub postboxes: PostboxesSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getRecipientDataRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetRecipientDataRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "safeId", prefix = "nsi1", default)]
	pub safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getRecipientDataResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetRecipientDataResponse {
	#[yaserde(rename = "recipient", prefix = "nsi1", default)]
	pub recipient: RecipientSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getIdentityDataRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetIdentityDataRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "safeId", prefix = "nsi1", default)]
	pub safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getIdentityDataResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetIdentityDataResponse {
	#[yaserde(rename = "identity", prefix = "nsi1", default)]
	pub identity: IdentitySoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getProcesscardsRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetProcesscardsRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getProcesscardsResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetProcesscardsResponse {
	#[yaserde(rename = "processcards", prefix = "nsi1", default)]
	pub processcards: ProcessCardsSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationResultRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetVerificationResultRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationResultResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetVerificationResultResponse {
	#[yaserde(rename = "verificationResult", prefix = "nsi1", default)]
	pub verification_result: Option<VerificationResultSoapDTO>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationConfigRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetVerificationConfigRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getVerificationConfigResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetVerificationConfigResponse {
	#[yaserde(rename = "verificationConfig", prefix = "nsi1", default)]
	pub verification_config: VerificationConfigSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "setProcesscardsRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SetProcesscardsRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "processcards", prefix = "nsi1", default)]
	pub processcards: ProcessCardsSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "setProcesscardsResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SetProcesscardsResponse {
	#[yaserde(rename = "processcardsSaved", prefix = "nsi1", default)]
	pub processcards_saved: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageJournalRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageJournalRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
	#[yaserde(rename = "from", prefix = "nsi1", default)]
	pub from: Option<String>, 
	#[yaserde(rename = "until", prefix = "nsi1", default)]
	pub until: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageJournalResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageJournalResponse {
	#[yaserde(rename = "messageJournals", prefix = "nsi1", default)]
	pub message_journals: JournalListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxJournalRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetPostboxJournalRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "from", prefix = "nsi1", default)]
	pub from: Option<String>, 
	#[yaserde(rename = "until", prefix = "nsi1", default)]
	pub until: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getPostboxJournalResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetPostboxJournalResponse {
	#[yaserde(rename = "postboxJournals", prefix = "nsi1", default)]
	pub postbox_journals: JournalListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAccessRightRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteAccessRightRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "recieverIdentitySafeId", prefix = "nsi1", default)]
	pub reciever_identity_safe_id: String, 
	#[yaserde(rename = "privilege", prefix = "nsi1", default)]
	pub privilege: PrivilegeTypeSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteAccessRightResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteAccessRightResponse {
	#[yaserde(rename = "accessRightDeleted", prefix = "nsi1", default)]
	pub access_right_deleted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeCommentRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RemoveCommentRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
	#[yaserde(rename = "commentId", prefix = "nsi1", default)]
	pub comment_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeCommentResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RemoveCommentResponse {
	#[yaserde(rename = "commentRemoved", prefix = "nsi1", default)]
	pub comment_removed: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateCommentRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct UpdateCommentRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "comment", prefix = "nsi1", default)]
	pub comment: CommentSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateCommentResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct UpdateCommentResponse {
	#[yaserde(rename = "commentUpdated", prefix = "nsi1", default)]
	pub comment_updated: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderOverviewRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetFolderOverviewRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "folderId", prefix = "nsi1", default)]
	pub folder_id: i64, 
	#[yaserde(rename = "onlyNew", prefix = "nsi1", default)]
	pub only_new: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFolderOverviewResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetFolderOverviewResponse {
	#[yaserde(rename = "folderOverview", prefix = "nsi1", default)]
	pub folder_overview: FolderOverviewSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "searchAddresseeRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SearchAddresseeRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "addresseeSearchParameter", prefix = "nsi1", default)]
	pub addressee_search_parameter: SearchAddresseeSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "searchAddresseeResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct SearchAddresseeResponse {
	#[yaserde(rename = "addressees", prefix = "nsi1", default)]
	pub addressees: IdentityListSoapDTO, 
	#[yaserde(rename = "warningMessages", prefix = "nsi1", default)]
	pub warning_messages: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFavouriteAddresseeRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetFavouriteAddresseeRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getFavouriteAddresseeResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetFavouriteAddresseeResponse {
	#[yaserde(rename = "favouriteAddressees", prefix = "nsi1", default)]
	pub favourite_addressees: IdentityListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteFavouriteAddresseeRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteFavouriteAddresseeRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "favouriteIdentitySafeId", prefix = "nsi1", default)]
	pub favourite_identity_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteFavouriteAddresseeResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteFavouriteAddresseeResponse {
	#[yaserde(rename = "favouriteDeleted", prefix = "nsi1", default)]
	pub favourite_deleted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToFolderRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct MoveMessageToFolderRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
	#[yaserde(rename = "folderId", prefix = "nsi1", default)]
	pub folder_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToFolderResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct MoveMessageToFolderResponse {
	#[yaserde(rename = "messageMoved", prefix = "nsi1", default)]
	pub message_moved: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToTrashRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct MoveMessageToTrashRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "moveMessageToTrashResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct MoveMessageToTrashResponse {
	#[yaserde(rename = "messageMoved", prefix = "nsi1", default)]
	pub message_moved: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "restoreMessageFromTrashRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RestoreMessageFromTrashRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "restoreMessageFromTrashResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RestoreMessageFromTrashResponse {
	#[yaserde(rename = "messageRestored", prefix = "nsi1", default)]
	pub message_restored: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteMessageRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteMessageRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "deleteMessageResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct DeleteMessageResponse {
	#[yaserde(rename = "messageDeleted", prefix = "nsi1", default)]
	pub message_deleted: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addFolderRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddFolderRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "folderName", prefix = "nsi1", default)]
	pub folder_name: String, 
	#[yaserde(rename = "parentFolderId", prefix = "nsi1", default)]
	pub parent_folder_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addFolderResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddFolderResponse {
	#[yaserde(rename = "newFolder", prefix = "nsi1", default)]
	pub new_folder: FolderSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateFolderRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct UpdateFolderRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "folderId", prefix = "nsi1", default)]
	pub folder_id: i64, 
	#[yaserde(rename = "newFolderName", prefix = "nsi1", default)]
	pub new_folder_name: Option<String>, 
	#[yaserde(rename = "newParentFolderId", prefix = "nsi1", default)]
	pub new_parent_folder_id: Option<i64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "updateFolderResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct UpdateFolderResponse {
	#[yaserde(rename = "folderUpdated", prefix = "nsi1", default)]
	pub folder_updated: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeFolderRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RemoveFolderRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "folderId", prefix = "nsi1", default)]
	pub folder_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "removeFolderResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RemoveFolderResponse {
	#[yaserde(rename = "folderRemoved", prefix = "nsi1", default)]
	pub folder_removed: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addCommentRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddCommentRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "comment", prefix = "nsi1", default)]
	pub comment: CommentSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "addCommentResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct AddCommentResponse {
	#[yaserde(rename = "newComment", prefix = "nsi1", default)]
	pub new_comment: CommentSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageConfigRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageConfigRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageConfigResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageConfigResponse {
	#[yaserde(rename = "messageConfig", prefix = "nsi1", default)]
	pub message_config: MessageConfigurationSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "activateAuthenticationRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct ActivateAuthenticationRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
	#[yaserde(rename = "securityToken", prefix = "nsi1", default)]
	pub security_token: SecurityTokenSoapDTO, 
	#[yaserde(rename = "signedPrivilege", prefix = "nsi1", default)]
	pub signed_privilege: SignedPrivilegeSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "activateAuthenticationResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct ActivateAuthenticationResponse {
	#[yaserde(rename = "authenticationActivated", prefix = "nsi1", default)]
	pub authentication_activated: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUnactivatedSecurityTokensRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetUnactivatedSecurityTokensRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "postboxSafeId", prefix = "nsi1", default)]
	pub postbox_safe_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getUnactivatedSecurityTokensResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetUnactivatedSecurityTokensResponse {
	#[yaserde(rename = "securityTokens", prefix = "nsi1", default)]
	pub security_tokens: Vec<SecurityTokenSoapDTO>, 
	#[yaserde(rename = "version", prefix = "nsi1", default)]
	pub version: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetAccessRightsRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetAccessRightsResponse {
	#[yaserde(rename = "accessRights", prefix = "nsi1", default)]
	pub access_rights: AccessRightListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsGrantedRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetAccessRightsGrantedRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getAccessRightsGrantedResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetAccessRightsGrantedResponse {
	#[yaserde(rename = "accessRights", prefix = "nsi1", default)]
	pub access_rights: AccessRightListSoapDTO, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageOverviewRequest",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageOverviewRequest {
	#[yaserde(rename = "sessionId", prefix = "nsi1", default)]
	pub session_id: String, 
	#[yaserde(rename = "messageId", prefix = "nsi1", default)]
	pub message_id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "getMessageOverviewResponse",
	namespace = "tns: http://brak.de/bea/application/dto/soap/types5",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct GetMessageOverviewResponse {
	#[yaserde(rename = "messageOverview", prefix = "nsi1", default)]
	pub message_overview: MessageOverviewSoapDTO, 
}
}

pub mod ports {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Fault",
	namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
	prefix = "soapenv",
)]
pub struct SoapBeaFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>, 
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>, 
	#[yaserde(rename = "BeaFault", default)]
	pub detail: Option<BeaFault>, 
}
pub type GetMessageRequest = messages::GetMessageRequest;

pub type GetMessageResponse = messages::GetMessageResponse;

pub type BeaFault = messages::BeaFault;

pub type CreateNewMessageRequest = messages::CreateNewMessageRequest;

pub type CreateNewMessageResponse = messages::CreateNewMessageResponse;

pub type SaveMessageRequest = messages::SaveMessageRequest;

pub type SaveMessageResponse = messages::SaveMessageResponse;

pub type SendMessageRequest = messages::SendMessageRequest;

pub type SendMessageResponse = messages::SendMessageResponse;

pub type GetFolderStructureRequest = messages::GetFolderStructureRequest;

pub type GetFolderStructureResponse = messages::GetFolderStructureResponse;

pub type GetFolderOverviewRequest = messages::GetFolderOverviewRequest;

pub type GetFolderOverviewResponse = messages::GetFolderOverviewResponse;

pub type SearchAddresseeRequest = messages::SearchAddresseeRequest;

pub type SearchAddresseeResponse = messages::SearchAddresseeResponse;

pub type GetFavouriteAddresseeRequest = messages::GetFavouriteAddresseeRequest;

pub type GetFavouriteAddresseeResponse = messages::GetFavouriteAddresseeResponse;

pub type DeleteFavouriteAddresseeRequest = messages::DeleteFavouriteAddresseeRequest;

pub type DeleteFavouriteAddresseeResponse = messages::DeleteFavouriteAddresseeResponse;

pub type MoveMessageToFolderRequest = messages::MoveMessageToFolderRequest;

pub type MoveMessageToFolderResponse = messages::MoveMessageToFolderResponse;

pub type MoveMessageToTrashRequest = messages::MoveMessageToTrashRequest;

pub type MoveMessageToTrashResponse = messages::MoveMessageToTrashResponse;

pub type RestoreMessageFromTrashRequest = messages::RestoreMessageFromTrashRequest;

pub type RestoreMessageFromTrashResponse = messages::RestoreMessageFromTrashResponse;

pub type DeleteMessageRequest = messages::DeleteMessageRequest;

pub type DeleteMessageResponse = messages::DeleteMessageResponse;

pub type AddFolderRequest = messages::AddFolderRequest;

pub type AddFolderResponse = messages::AddFolderResponse;

pub type UpdateFolderRequest = messages::UpdateFolderRequest;

pub type UpdateFolderResponse = messages::UpdateFolderResponse;

pub type RemoveFolderRequest = messages::RemoveFolderRequest;

pub type RemoveFolderResponse = messages::RemoveFolderResponse;

pub type AddCommentRequest = messages::AddCommentRequest;

pub type AddCommentResponse = messages::AddCommentResponse;

pub type UpdateCommentRequest = messages::UpdateCommentRequest;

pub type UpdateCommentResponse = messages::UpdateCommentResponse;

pub type RemoveCommentRequest = messages::RemoveCommentRequest;

pub type RemoveCommentResponse = messages::RemoveCommentResponse;

pub type GetVerificationResultRequest = messages::GetVerificationResultRequest;

pub type GetVerificationResultResponse = messages::GetVerificationResultResponse;

pub type GetVerificationConfigRequest = messages::GetVerificationConfigRequest;

pub type GetVerificationConfigResponse = messages::GetVerificationConfigResponse;

pub type GetMessageConfigRequest = messages::GetMessageConfigRequest;

pub type GetMessageConfigResponse = messages::GetMessageConfigResponse;

pub type LoginUserRequest = messages::LoginUserRequest;

pub type LoginUserResponse = messages::LoginUserResponse;

pub type LogoutUserRequest = messages::LogoutUserRequest;

pub type LogoutUserResponse = messages::LogoutUserResponse;

pub type StartLoginUserRequest = messages::StartLoginUserRequest;

pub type StartLoginUserResponse = messages::StartLoginUserResponse;

pub type GetConfigurationRequest = messages::GetConfigurationRequest;

pub type GetConfigurationResponse = messages::GetConfigurationResponse;

pub type GetUserNameRequest = messages::GetUserNameRequest;

pub type GetUserNameResponse = messages::GetUserNameResponse;

pub type GetUserJournalRequest = messages::GetUserJournalRequest;

pub type GetUserJournalResponse = messages::GetUserJournalResponse;

pub type DeleteAuthentificationRequest = messages::DeleteAuthentificationRequest;

pub type DeleteAuthentificationResponse = messages::DeleteAuthentificationResponse;

pub type AddAuthentificationRequest = messages::AddAuthentificationRequest;

pub type AddAuthentificationResponse = messages::AddAuthentificationResponse;

pub type StartRegistrationRequest = messages::StartRegistrationRequest;

pub type StartRegistrationResponse = messages::StartRegistrationResponse;

pub type FinishRegistrationRequest = messages::FinishRegistrationRequest;

pub type FinishRegistrationResponse = messages::FinishRegistrationResponse;

pub type GetAddressbookRequest = messages::GetAddressbookRequest;

pub type GetAddressbookResponse = messages::GetAddressbookResponse;

pub type DeleteAddressbookEntryRequest = messages::DeleteAddressbookEntryRequest;

pub type DeleteAddressbookEntryResponse = messages::DeleteAddressbookEntryResponse;

pub type AddAddressbookEntryRequest = messages::AddAddressbookEntryRequest;

pub type AddAddressbookEntryResponse = messages::AddAddressbookEntryResponse;

pub type GetRecipientDataRequest = messages::GetRecipientDataRequest;

pub type GetRecipientDataResponse = messages::GetRecipientDataResponse;

pub type GetIdentityDataRequest = messages::GetIdentityDataRequest;

pub type GetIdentityDataResponse = messages::GetIdentityDataResponse;

pub type AddUserRequest = messages::AddUserRequest;

pub type AddUserResponse = messages::AddUserResponse;

pub type UpdateUserRequest = messages::UpdateUserRequest;

pub type UpdateUserResponse = messages::UpdateUserResponse;

pub type DeleteUserRequest = messages::DeleteUserRequest;

pub type DeleteUserResponse = messages::DeleteUserResponse;

pub type AddAccessRightRequest = messages::AddAccessRightRequest;

pub type AddAccessRightResponse = messages::AddAccessRightResponse;

pub type DeleteAccessRightRequest = messages::DeleteAccessRightRequest;

pub type DeleteAccessRightResponse = messages::DeleteAccessRightResponse;

pub type GetPostboxJournalRequest = messages::GetPostboxJournalRequest;

pub type GetPostboxJournalResponse = messages::GetPostboxJournalResponse;

pub type GetMessageJournalRequest = messages::GetMessageJournalRequest;

pub type GetMessageJournalResponse = messages::GetMessageJournalResponse;

pub type GetPostboxesRequest = messages::GetPostboxesRequest;

pub type GetPostboxesResponse = messages::GetPostboxesResponse;

pub type GetProcesscardsRequest = messages::GetProcesscardsRequest;

pub type GetProcesscardsResponse = messages::GetProcesscardsResponse;

pub type SetProcesscardsRequest = messages::SetProcesscardsRequest;

pub type SetProcesscardsResponse = messages::SetProcesscardsResponse;

pub type ActivateAuthenticationRequest = messages::ActivateAuthenticationRequest;

pub type ActivateAuthenticationResponse = messages::ActivateAuthenticationResponse;

pub type GetUnactivatedSecurityTokensRequest = messages::GetUnactivatedSecurityTokensRequest;

pub type GetUnactivatedSecurityTokensResponse = messages::GetUnactivatedSecurityTokensResponse;

pub type GetAccessRightsRequest = messages::GetAccessRightsRequest;

pub type GetAccessRightsResponse = messages::GetAccessRightsResponse;

pub type GetAccessRightsGrantedRequest = messages::GetAccessRightsGrantedRequest;

pub type GetAccessRightsGrantedResponse = messages::GetAccessRightsGrantedResponse;

pub type GetMessageOverviewRequest = messages::GetMessageOverviewRequest;

pub type GetMessageOverviewResponse = messages::GetMessageOverviewResponse;

#[async_trait]
pub trait BeAServiceV5PortType {
	async fn get_message (&self, get_message_request: GetMessageRequest) -> Result<GetMessageResponse,Option<SoapBeaFault>>;
	async fn create_new_message (&self, create_new_message_request: CreateNewMessageRequest) -> Result<CreateNewMessageResponse,Option<SoapBeaFault>>;
	async fn save_message (&self, save_message_request: SaveMessageRequest) -> Result<SaveMessageResponse,Option<SoapBeaFault>>;
	async fn send_message (&self, send_message_request: SendMessageRequest) -> Result<SendMessageResponse,Option<SoapBeaFault>>;
	async fn get_folder_structure (&self, get_folder_structure_request: GetFolderStructureRequest) -> Result<GetFolderStructureResponse,Option<SoapBeaFault>>;
	async fn get_folder_overview (&self, get_folder_overview_request: GetFolderOverviewRequest) -> Result<GetFolderOverviewResponse,Option<SoapBeaFault>>;
	async fn search_addressee (&self, search_addressee_request: SearchAddresseeRequest) -> Result<SearchAddresseeResponse,Option<SoapBeaFault>>;
	async fn get_favourite_addressee (&self, get_favourite_addressee_request: GetFavouriteAddresseeRequest) -> Result<GetFavouriteAddresseeResponse,Option<SoapBeaFault>>;
	async fn delete_favourite_addressee (&self, delete_favourite_addressee_request: DeleteFavouriteAddresseeRequest) -> Result<DeleteFavouriteAddresseeResponse,Option<SoapBeaFault>>;
	async fn move_message_to_folder (&self, move_message_to_folder_request: MoveMessageToFolderRequest) -> Result<MoveMessageToFolderResponse,Option<SoapBeaFault>>;
	async fn move_message_to_trash (&self, move_message_to_trash_request: MoveMessageToTrashRequest) -> Result<MoveMessageToTrashResponse,Option<SoapBeaFault>>;
	async fn restore_message_from_trash (&self, restore_message_from_trash_request: RestoreMessageFromTrashRequest) -> Result<RestoreMessageFromTrashResponse,Option<SoapBeaFault>>;
	async fn delete_message (&self, delete_message_request: DeleteMessageRequest) -> Result<DeleteMessageResponse,Option<SoapBeaFault>>;
	async fn add_folder (&self, add_folder_request: AddFolderRequest) -> Result<AddFolderResponse,Option<SoapBeaFault>>;
	async fn update_folder (&self, update_folder_request: UpdateFolderRequest) -> Result<UpdateFolderResponse,Option<SoapBeaFault>>;
	async fn remove_folder (&self, remove_folder_request: RemoveFolderRequest) -> Result<RemoveFolderResponse,Option<SoapBeaFault>>;
	async fn add_comment (&self, add_comment_request: AddCommentRequest) -> Result<AddCommentResponse,Option<SoapBeaFault>>;
	async fn update_comment (&self, update_comment_request: UpdateCommentRequest) -> Result<UpdateCommentResponse,Option<SoapBeaFault>>;
	async fn remove_comment (&self, remove_comment_request: RemoveCommentRequest) -> Result<RemoveCommentResponse,Option<SoapBeaFault>>;
	async fn get_verification_result (&self, get_verification_result_request: GetVerificationResultRequest) -> Result<GetVerificationResultResponse,Option<SoapBeaFault>>;
	async fn get_verification_config (&self, get_verification_config_request: GetVerificationConfigRequest) -> Result<GetVerificationConfigResponse,Option<SoapBeaFault>>;
	async fn get_message_config (&self, get_message_config_request: GetMessageConfigRequest) -> Result<GetMessageConfigResponse,Option<SoapBeaFault>>;
	async fn login_user (&self, login_user_request: LoginUserRequest) -> Result<LoginUserResponse,Option<SoapBeaFault>>;
	async fn logout_user (&self, logout_user_request: LogoutUserRequest) -> Result<LogoutUserResponse,Option<SoapBeaFault>>;
	async fn start_login_user (&self, start_login_user_request: StartLoginUserRequest) -> Result<StartLoginUserResponse,Option<SoapBeaFault>>;
	async fn get_configuration (&self, get_configuration_request: GetConfigurationRequest) -> Result<GetConfigurationResponse,Option<SoapBeaFault>>;
	async fn get_user_name (&self, get_user_name_request: GetUserNameRequest) -> Result<GetUserNameResponse,Option<SoapBeaFault>>;
	async fn get_user_journal (&self, get_user_journal_request: GetUserJournalRequest) -> Result<GetUserJournalResponse,Option<SoapBeaFault>>;
	async fn delete_authentification (&self, delete_authentification_request: DeleteAuthentificationRequest) -> Result<DeleteAuthentificationResponse,Option<SoapBeaFault>>;
	async fn add_authentification (&self, add_authentification_request: AddAuthentificationRequest) -> Result<AddAuthentificationResponse,Option<SoapBeaFault>>;
	async fn start_registration (&self, start_registration_request: StartRegistrationRequest) -> Result<StartRegistrationResponse,Option<SoapBeaFault>>;
	async fn finish_registration (&self, finish_registration_request: FinishRegistrationRequest) -> Result<FinishRegistrationResponse,Option<SoapBeaFault>>;
	async fn get_addressbook (&self, get_addressbook_request: GetAddressbookRequest) -> Result<GetAddressbookResponse,Option<SoapBeaFault>>;
	async fn delete_addressbook_entry (&self, delete_addressbook_entry_request: DeleteAddressbookEntryRequest) -> Result<DeleteAddressbookEntryResponse,Option<SoapBeaFault>>;
	async fn add_addressbook_entry (&self, add_addressbook_entry_request: AddAddressbookEntryRequest) -> Result<AddAddressbookEntryResponse,Option<SoapBeaFault>>;
	async fn get_recipient_data (&self, get_recipient_data_request: GetRecipientDataRequest) -> Result<GetRecipientDataResponse,Option<SoapBeaFault>>;
	async fn get_identity_data (&self, get_identity_data_request: GetIdentityDataRequest) -> Result<GetIdentityDataResponse,Option<SoapBeaFault>>;
	async fn add_user (&self, add_user_request: AddUserRequest) -> Result<AddUserResponse,Option<SoapBeaFault>>;
	async fn update_user (&self, update_user_request: UpdateUserRequest) -> Result<UpdateUserResponse,Option<SoapBeaFault>>;
	async fn delete_user (&self, delete_user_request: DeleteUserRequest) -> Result<DeleteUserResponse,Option<SoapBeaFault>>;
	async fn add_access_right (&self, add_access_right_request: AddAccessRightRequest) -> Result<AddAccessRightResponse,Option<SoapBeaFault>>;
	async fn delete_access_right (&self, delete_access_right_request: DeleteAccessRightRequest) -> Result<DeleteAccessRightResponse,Option<SoapBeaFault>>;
	async fn get_postbox_journal (&self, get_postbox_journal_request: GetPostboxJournalRequest) -> Result<GetPostboxJournalResponse,Option<SoapBeaFault>>;
	async fn get_message_journal (&self, get_message_journal_request: GetMessageJournalRequest) -> Result<GetMessageJournalResponse,Option<SoapBeaFault>>;
	async fn get_postboxes (&self, get_postboxes_request: GetPostboxesRequest) -> Result<GetPostboxesResponse,Option<SoapBeaFault>>;
	async fn get_processcards (&self, get_processcards_request: GetProcesscardsRequest) -> Result<GetProcesscardsResponse,Option<SoapBeaFault>>;
	async fn set_processcards (&self, set_processcards_request: SetProcesscardsRequest) -> Result<SetProcesscardsResponse,Option<SoapBeaFault>>;
	async fn activate_authentication (&self, activate_authentication_request: ActivateAuthenticationRequest) -> Result<ActivateAuthenticationResponse,Option<SoapBeaFault>>;
	async fn get_unactivated_security_tokens (&self, get_unactivated_security_tokens_request: GetUnactivatedSecurityTokensRequest) -> Result<GetUnactivatedSecurityTokensResponse,Option<SoapBeaFault>>;
	async fn get_access_rights (&self, get_access_rights_request: GetAccessRightsRequest) -> Result<GetAccessRightsResponse,Option<SoapBeaFault>>;
	async fn get_access_rights_granted (&self, get_access_rights_granted_request: GetAccessRightsGrantedRequest) -> Result<GetAccessRightsGrantedResponse,Option<SoapBeaFault>>;
	async fn get_message_overview (&self, get_message_overview_request: GetMessageOverviewRequest) -> Result<GetMessageOverviewResponse,Option<SoapBeaFault>>;
}
}

pub mod bindings {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            
            impl BeAServiceV5HttpBinding {
                async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
                    let body = to_string(request).expect("failed to generate xml");
                    debug!("SOAP Request: {}", body);
                    let mut req = self
                        .client
                        .post(&self.url)
                        .body(body)
                        .header("Content-Type", "text/xml")
                        .header("Soapaction", action);
                    if let Some(credentials) = &self.credentials {
                        req = req.basic_auth(
                            credentials.0.to_string(),
                            Option::Some(credentials.1.to_string()),
                        );
                    }
                    let res = req.send().await?;
                    let status = res.status();
                    debug!("SOAP Status: {}", status);
                    let txt = res.text().await.unwrap_or_default();
                    debug!("SOAP Response: {}", txt);
                    Ok((status, txt))
                }
            }
            #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageRequest {
                        #[yaserde(rename = "getMessage", default)]
                        pub body: ports::GetMessageRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageRequest,
        }
        
        impl GetMessageRequestSoapEnvelope {
            pub fn new(body: SoapGetMessageRequest) -> Self {
                GetMessageRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageResponse {
                    #[yaserde(rename = "GetMessageResponse", default)]
                    pub body: ports::GetMessageResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageResponse,
        }
        
        impl GetMessageResponseSoapEnvelope {
            pub fn new(body: SoapGetMessageResponse) -> Self {
                GetMessageResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapCreateNewMessageRequest {
                        #[yaserde(rename = "createNewMessage", default)]
                        pub body: ports::CreateNewMessageRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct CreateNewMessageRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapCreateNewMessageRequest,
        }
        
        impl CreateNewMessageRequestSoapEnvelope {
            pub fn new(body: SoapCreateNewMessageRequest) -> Self {
                CreateNewMessageRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapCreateNewMessageResponse {
                    #[yaserde(rename = "CreateNewMessageResponse", default)]
                    pub body: ports::CreateNewMessageResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct CreateNewMessageResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapCreateNewMessageResponse,
        }
        
        impl CreateNewMessageResponseSoapEnvelope {
            pub fn new(body: SoapCreateNewMessageResponse) -> Self {
                CreateNewMessageResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSaveMessageRequest {
                        #[yaserde(rename = "saveMessage", default)]
                        pub body: ports::SaveMessageRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SaveMessageRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSaveMessageRequest,
        }
        
        impl SaveMessageRequestSoapEnvelope {
            pub fn new(body: SoapSaveMessageRequest) -> Self {
                SaveMessageRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSaveMessageResponse {
                    #[yaserde(rename = "SaveMessageResponse", default)]
                    pub body: ports::SaveMessageResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SaveMessageResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSaveMessageResponse,
        }
        
        impl SaveMessageResponseSoapEnvelope {
            pub fn new(body: SoapSaveMessageResponse) -> Self {
                SaveMessageResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSendMessageRequest {
                        #[yaserde(rename = "sendMessage", default)]
                        pub body: ports::SendMessageRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SendMessageRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSendMessageRequest,
        }
        
        impl SendMessageRequestSoapEnvelope {
            pub fn new(body: SoapSendMessageRequest) -> Self {
                SendMessageRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSendMessageResponse {
                    #[yaserde(rename = "SendMessageResponse", default)]
                    pub body: ports::SendMessageResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SendMessageResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSendMessageResponse,
        }
        
        impl SendMessageResponseSoapEnvelope {
            pub fn new(body: SoapSendMessageResponse) -> Self {
                SendMessageResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetFolderStructureRequest {
                        #[yaserde(rename = "getFolderStructure", default)]
                        pub body: ports::GetFolderStructureRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetFolderStructureRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetFolderStructureRequest,
        }
        
        impl GetFolderStructureRequestSoapEnvelope {
            pub fn new(body: SoapGetFolderStructureRequest) -> Self {
                GetFolderStructureRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetFolderStructureResponse {
                    #[yaserde(rename = "GetFolderStructureResponse", default)]
                    pub body: ports::GetFolderStructureResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetFolderStructureResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetFolderStructureResponse,
        }
        
        impl GetFolderStructureResponseSoapEnvelope {
            pub fn new(body: SoapGetFolderStructureResponse) -> Self {
                GetFolderStructureResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetFolderOverviewRequest {
                        #[yaserde(rename = "getFolderOverview", default)]
                        pub body: ports::GetFolderOverviewRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetFolderOverviewRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetFolderOverviewRequest,
        }
        
        impl GetFolderOverviewRequestSoapEnvelope {
            pub fn new(body: SoapGetFolderOverviewRequest) -> Self {
                GetFolderOverviewRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetFolderOverviewResponse {
                    #[yaserde(rename = "GetFolderOverviewResponse", default)]
                    pub body: ports::GetFolderOverviewResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetFolderOverviewResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetFolderOverviewResponse,
        }
        
        impl GetFolderOverviewResponseSoapEnvelope {
            pub fn new(body: SoapGetFolderOverviewResponse) -> Self {
                GetFolderOverviewResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSearchAddresseeRequest {
                        #[yaserde(rename = "searchAddressee", default)]
                        pub body: ports::SearchAddresseeRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SearchAddresseeRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSearchAddresseeRequest,
        }
        
        impl SearchAddresseeRequestSoapEnvelope {
            pub fn new(body: SoapSearchAddresseeRequest) -> Self {
                SearchAddresseeRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSearchAddresseeResponse {
                    #[yaserde(rename = "SearchAddresseeResponse", default)]
                    pub body: ports::SearchAddresseeResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SearchAddresseeResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSearchAddresseeResponse,
        }
        
        impl SearchAddresseeResponseSoapEnvelope {
            pub fn new(body: SoapSearchAddresseeResponse) -> Self {
                SearchAddresseeResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetFavouriteAddresseeRequest {
                        #[yaserde(rename = "getFavouriteAddressee", default)]
                        pub body: ports::GetFavouriteAddresseeRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetFavouriteAddresseeRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetFavouriteAddresseeRequest,
        }
        
        impl GetFavouriteAddresseeRequestSoapEnvelope {
            pub fn new(body: SoapGetFavouriteAddresseeRequest) -> Self {
                GetFavouriteAddresseeRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetFavouriteAddresseeResponse {
                    #[yaserde(rename = "GetFavouriteAddresseeResponse", default)]
                    pub body: ports::GetFavouriteAddresseeResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetFavouriteAddresseeResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetFavouriteAddresseeResponse,
        }
        
        impl GetFavouriteAddresseeResponseSoapEnvelope {
            pub fn new(body: SoapGetFavouriteAddresseeResponse) -> Self {
                GetFavouriteAddresseeResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteFavouriteAddresseeRequest {
                        #[yaserde(rename = "deleteFavouriteAddressee", default)]
                        pub body: ports::DeleteFavouriteAddresseeRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteFavouriteAddresseeRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteFavouriteAddresseeRequest,
        }
        
        impl DeleteFavouriteAddresseeRequestSoapEnvelope {
            pub fn new(body: SoapDeleteFavouriteAddresseeRequest) -> Self {
                DeleteFavouriteAddresseeRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteFavouriteAddresseeResponse {
                    #[yaserde(rename = "DeleteFavouriteAddresseeResponse", default)]
                    pub body: ports::DeleteFavouriteAddresseeResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteFavouriteAddresseeResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteFavouriteAddresseeResponse,
        }
        
        impl DeleteFavouriteAddresseeResponseSoapEnvelope {
            pub fn new(body: SoapDeleteFavouriteAddresseeResponse) -> Self {
                DeleteFavouriteAddresseeResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapMoveMessageToFolderRequest {
                        #[yaserde(rename = "moveMessageToFolder", default)]
                        pub body: ports::MoveMessageToFolderRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct MoveMessageToFolderRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapMoveMessageToFolderRequest,
        }
        
        impl MoveMessageToFolderRequestSoapEnvelope {
            pub fn new(body: SoapMoveMessageToFolderRequest) -> Self {
                MoveMessageToFolderRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapMoveMessageToFolderResponse {
                    #[yaserde(rename = "MoveMessageToFolderResponse", default)]
                    pub body: ports::MoveMessageToFolderResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct MoveMessageToFolderResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapMoveMessageToFolderResponse,
        }
        
        impl MoveMessageToFolderResponseSoapEnvelope {
            pub fn new(body: SoapMoveMessageToFolderResponse) -> Self {
                MoveMessageToFolderResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapMoveMessageToTrashRequest {
                        #[yaserde(rename = "moveMessageToTrash", default)]
                        pub body: ports::MoveMessageToTrashRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct MoveMessageToTrashRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapMoveMessageToTrashRequest,
        }
        
        impl MoveMessageToTrashRequestSoapEnvelope {
            pub fn new(body: SoapMoveMessageToTrashRequest) -> Self {
                MoveMessageToTrashRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapMoveMessageToTrashResponse {
                    #[yaserde(rename = "MoveMessageToTrashResponse", default)]
                    pub body: ports::MoveMessageToTrashResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct MoveMessageToTrashResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapMoveMessageToTrashResponse,
        }
        
        impl MoveMessageToTrashResponseSoapEnvelope {
            pub fn new(body: SoapMoveMessageToTrashResponse) -> Self {
                MoveMessageToTrashResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapRestoreMessageFromTrashRequest {
                        #[yaserde(rename = "restoreMessageFromTrash", default)]
                        pub body: ports::RestoreMessageFromTrashRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct RestoreMessageFromTrashRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapRestoreMessageFromTrashRequest,
        }
        
        impl RestoreMessageFromTrashRequestSoapEnvelope {
            pub fn new(body: SoapRestoreMessageFromTrashRequest) -> Self {
                RestoreMessageFromTrashRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapRestoreMessageFromTrashResponse {
                    #[yaserde(rename = "RestoreMessageFromTrashResponse", default)]
                    pub body: ports::RestoreMessageFromTrashResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct RestoreMessageFromTrashResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapRestoreMessageFromTrashResponse,
        }
        
        impl RestoreMessageFromTrashResponseSoapEnvelope {
            pub fn new(body: SoapRestoreMessageFromTrashResponse) -> Self {
                RestoreMessageFromTrashResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteMessageRequest {
                        #[yaserde(rename = "deleteMessage", default)]
                        pub body: ports::DeleteMessageRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteMessageRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteMessageRequest,
        }
        
        impl DeleteMessageRequestSoapEnvelope {
            pub fn new(body: SoapDeleteMessageRequest) -> Self {
                DeleteMessageRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteMessageResponse {
                    #[yaserde(rename = "DeleteMessageResponse", default)]
                    pub body: ports::DeleteMessageResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteMessageResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteMessageResponse,
        }
        
        impl DeleteMessageResponseSoapEnvelope {
            pub fn new(body: SoapDeleteMessageResponse) -> Self {
                DeleteMessageResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddFolderRequest {
                        #[yaserde(rename = "addFolder", default)]
                        pub body: ports::AddFolderRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddFolderRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddFolderRequest,
        }
        
        impl AddFolderRequestSoapEnvelope {
            pub fn new(body: SoapAddFolderRequest) -> Self {
                AddFolderRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddFolderResponse {
                    #[yaserde(rename = "AddFolderResponse", default)]
                    pub body: ports::AddFolderResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddFolderResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddFolderResponse,
        }
        
        impl AddFolderResponseSoapEnvelope {
            pub fn new(body: SoapAddFolderResponse) -> Self {
                AddFolderResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapUpdateFolderRequest {
                        #[yaserde(rename = "updateFolder", default)]
                        pub body: ports::UpdateFolderRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct UpdateFolderRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapUpdateFolderRequest,
        }
        
        impl UpdateFolderRequestSoapEnvelope {
            pub fn new(body: SoapUpdateFolderRequest) -> Self {
                UpdateFolderRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapUpdateFolderResponse {
                    #[yaserde(rename = "UpdateFolderResponse", default)]
                    pub body: ports::UpdateFolderResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct UpdateFolderResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapUpdateFolderResponse,
        }
        
        impl UpdateFolderResponseSoapEnvelope {
            pub fn new(body: SoapUpdateFolderResponse) -> Self {
                UpdateFolderResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapRemoveFolderRequest {
                        #[yaserde(rename = "removeFolder", default)]
                        pub body: ports::RemoveFolderRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct RemoveFolderRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapRemoveFolderRequest,
        }
        
        impl RemoveFolderRequestSoapEnvelope {
            pub fn new(body: SoapRemoveFolderRequest) -> Self {
                RemoveFolderRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapRemoveFolderResponse {
                    #[yaserde(rename = "RemoveFolderResponse", default)]
                    pub body: ports::RemoveFolderResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct RemoveFolderResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapRemoveFolderResponse,
        }
        
        impl RemoveFolderResponseSoapEnvelope {
            pub fn new(body: SoapRemoveFolderResponse) -> Self {
                RemoveFolderResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddCommentRequest {
                        #[yaserde(rename = "addComment", default)]
                        pub body: ports::AddCommentRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddCommentRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddCommentRequest,
        }
        
        impl AddCommentRequestSoapEnvelope {
            pub fn new(body: SoapAddCommentRequest) -> Self {
                AddCommentRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddCommentResponse {
                    #[yaserde(rename = "AddCommentResponse", default)]
                    pub body: ports::AddCommentResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddCommentResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddCommentResponse,
        }
        
        impl AddCommentResponseSoapEnvelope {
            pub fn new(body: SoapAddCommentResponse) -> Self {
                AddCommentResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapUpdateCommentRequest {
                        #[yaserde(rename = "updateComment", default)]
                        pub body: ports::UpdateCommentRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct UpdateCommentRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapUpdateCommentRequest,
        }
        
        impl UpdateCommentRequestSoapEnvelope {
            pub fn new(body: SoapUpdateCommentRequest) -> Self {
                UpdateCommentRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapUpdateCommentResponse {
                    #[yaserde(rename = "UpdateCommentResponse", default)]
                    pub body: ports::UpdateCommentResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct UpdateCommentResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapUpdateCommentResponse,
        }
        
        impl UpdateCommentResponseSoapEnvelope {
            pub fn new(body: SoapUpdateCommentResponse) -> Self {
                UpdateCommentResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapRemoveCommentRequest {
                        #[yaserde(rename = "removeComment", default)]
                        pub body: ports::RemoveCommentRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct RemoveCommentRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapRemoveCommentRequest,
        }
        
        impl RemoveCommentRequestSoapEnvelope {
            pub fn new(body: SoapRemoveCommentRequest) -> Self {
                RemoveCommentRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapRemoveCommentResponse {
                    #[yaserde(rename = "RemoveCommentResponse", default)]
                    pub body: ports::RemoveCommentResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct RemoveCommentResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapRemoveCommentResponse,
        }
        
        impl RemoveCommentResponseSoapEnvelope {
            pub fn new(body: SoapRemoveCommentResponse) -> Self {
                RemoveCommentResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetUserNameRequest {
                        #[yaserde(rename = "getUserName", default)]
                        pub body: ports::GetUserNameRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetUserNameRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetUserNameRequest,
        }
        
        impl GetUserNameRequestSoapEnvelope {
            pub fn new(body: SoapGetUserNameRequest) -> Self {
                GetUserNameRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetUserNameResponse {
                    #[yaserde(rename = "GetUserNameResponse", default)]
                    pub body: ports::GetUserNameResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetUserNameResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetUserNameResponse,
        }
        
        impl GetUserNameResponseSoapEnvelope {
            pub fn new(body: SoapGetUserNameResponse) -> Self {
                GetUserNameResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetUserJournalRequest {
                        #[yaserde(rename = "getUserJournal", default)]
                        pub body: ports::GetUserJournalRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetUserJournalRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetUserJournalRequest,
        }
        
        impl GetUserJournalRequestSoapEnvelope {
            pub fn new(body: SoapGetUserJournalRequest) -> Self {
                GetUserJournalRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetUserJournalResponse {
                    #[yaserde(rename = "GetUserJournalResponse", default)]
                    pub body: ports::GetUserJournalResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetUserJournalResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetUserJournalResponse,
        }
        
        impl GetUserJournalResponseSoapEnvelope {
            pub fn new(body: SoapGetUserJournalResponse) -> Self {
                GetUserJournalResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteAuthentificationRequest {
                        #[yaserde(rename = "deleteAuthentification", default)]
                        pub body: ports::DeleteAuthentificationRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteAuthentificationRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteAuthentificationRequest,
        }
        
        impl DeleteAuthentificationRequestSoapEnvelope {
            pub fn new(body: SoapDeleteAuthentificationRequest) -> Self {
                DeleteAuthentificationRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteAuthentificationResponse {
                    #[yaserde(rename = "DeleteAuthentificationResponse", default)]
                    pub body: ports::DeleteAuthentificationResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteAuthentificationResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteAuthentificationResponse,
        }
        
        impl DeleteAuthentificationResponseSoapEnvelope {
            pub fn new(body: SoapDeleteAuthentificationResponse) -> Self {
                DeleteAuthentificationResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddAuthentificationRequest {
                        #[yaserde(rename = "addAuthentification", default)]
                        pub body: ports::AddAuthentificationRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddAuthentificationRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddAuthentificationRequest,
        }
        
        impl AddAuthentificationRequestSoapEnvelope {
            pub fn new(body: SoapAddAuthentificationRequest) -> Self {
                AddAuthentificationRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddAuthentificationResponse {
                    #[yaserde(rename = "AddAuthentificationResponse", default)]
                    pub body: ports::AddAuthentificationResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddAuthentificationResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddAuthentificationResponse,
        }
        
        impl AddAuthentificationResponseSoapEnvelope {
            pub fn new(body: SoapAddAuthentificationResponse) -> Self {
                AddAuthentificationResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapStartRegistrationRequest {
                        #[yaserde(rename = "startRegistration", default)]
                        pub body: ports::StartRegistrationRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct StartRegistrationRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapStartRegistrationRequest,
        }
        
        impl StartRegistrationRequestSoapEnvelope {
            pub fn new(body: SoapStartRegistrationRequest) -> Self {
                StartRegistrationRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapStartRegistrationResponse {
                    #[yaserde(rename = "StartRegistrationResponse", default)]
                    pub body: ports::StartRegistrationResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct StartRegistrationResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapStartRegistrationResponse,
        }
        
        impl StartRegistrationResponseSoapEnvelope {
            pub fn new(body: SoapStartRegistrationResponse) -> Self {
                StartRegistrationResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFinishRegistrationRequest {
                        #[yaserde(rename = "finishRegistration", default)]
                        pub body: ports::FinishRegistrationRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FinishRegistrationRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFinishRegistrationRequest,
        }
        
        impl FinishRegistrationRequestSoapEnvelope {
            pub fn new(body: SoapFinishRegistrationRequest) -> Self {
                FinishRegistrationRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFinishRegistrationResponse {
                    #[yaserde(rename = "FinishRegistrationResponse", default)]
                    pub body: ports::FinishRegistrationResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FinishRegistrationResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFinishRegistrationResponse,
        }
        
        impl FinishRegistrationResponseSoapEnvelope {
            pub fn new(body: SoapFinishRegistrationResponse) -> Self {
                FinishRegistrationResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapLoginUserRequest {
                        #[yaserde(rename = "loginUser", default)]
                        pub body: ports::LoginUserRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct LoginUserRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapLoginUserRequest,
        }
        
        impl LoginUserRequestSoapEnvelope {
            pub fn new(body: SoapLoginUserRequest) -> Self {
                LoginUserRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapLoginUserResponse {
                    #[yaserde(rename = "LoginUserResponse", default)]
                    pub body: ports::LoginUserResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct LoginUserResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapLoginUserResponse,
        }
        
        impl LoginUserResponseSoapEnvelope {
            pub fn new(body: SoapLoginUserResponse) -> Self {
                LoginUserResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapLogoutUserRequest {
                        #[yaserde(rename = "logoutUser", default)]
                        pub body: ports::LogoutUserRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct LogoutUserRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapLogoutUserRequest,
        }
        
        impl LogoutUserRequestSoapEnvelope {
            pub fn new(body: SoapLogoutUserRequest) -> Self {
                LogoutUserRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapLogoutUserResponse {
                    #[yaserde(rename = "LogoutUserResponse", default)]
                    pub body: ports::LogoutUserResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct LogoutUserResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapLogoutUserResponse,
        }
        
        impl LogoutUserResponseSoapEnvelope {
            pub fn new(body: SoapLogoutUserResponse) -> Self {
                LogoutUserResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapStartLoginUserRequest {
                        #[yaserde(rename = "startLoginUser", default)]
                        pub body: ports::StartLoginUserRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct StartLoginUserRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapStartLoginUserRequest,
        }
        
        impl StartLoginUserRequestSoapEnvelope {
            pub fn new(body: SoapStartLoginUserRequest) -> Self {
                StartLoginUserRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapStartLoginUserResponse {
                    #[yaserde(rename = "StartLoginUserResponse", default)]
                    pub body: ports::StartLoginUserResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct StartLoginUserResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapStartLoginUserResponse,
        }
        
        impl StartLoginUserResponseSoapEnvelope {
            pub fn new(body: SoapStartLoginUserResponse) -> Self {
                StartLoginUserResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddUserRequest {
                        #[yaserde(rename = "addUser", default)]
                        pub body: ports::AddUserRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddUserRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddUserRequest,
        }
        
        impl AddUserRequestSoapEnvelope {
            pub fn new(body: SoapAddUserRequest) -> Self {
                AddUserRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddUserResponse {
                    #[yaserde(rename = "AddUserResponse", default)]
                    pub body: ports::AddUserResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddUserResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddUserResponse,
        }
        
        impl AddUserResponseSoapEnvelope {
            pub fn new(body: SoapAddUserResponse) -> Self {
                AddUserResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapUpdateUserRequest {
                        #[yaserde(rename = "updateUser", default)]
                        pub body: ports::UpdateUserRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct UpdateUserRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapUpdateUserRequest,
        }
        
        impl UpdateUserRequestSoapEnvelope {
            pub fn new(body: SoapUpdateUserRequest) -> Self {
                UpdateUserRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapUpdateUserResponse {
                    #[yaserde(rename = "UpdateUserResponse", default)]
                    pub body: ports::UpdateUserResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct UpdateUserResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapUpdateUserResponse,
        }
        
        impl UpdateUserResponseSoapEnvelope {
            pub fn new(body: SoapUpdateUserResponse) -> Self {
                UpdateUserResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteUserRequest {
                        #[yaserde(rename = "deleteUser", default)]
                        pub body: ports::DeleteUserRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteUserRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteUserRequest,
        }
        
        impl DeleteUserRequestSoapEnvelope {
            pub fn new(body: SoapDeleteUserRequest) -> Self {
                DeleteUserRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteUserResponse {
                    #[yaserde(rename = "DeleteUserResponse", default)]
                    pub body: ports::DeleteUserResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteUserResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteUserResponse,
        }
        
        impl DeleteUserResponseSoapEnvelope {
            pub fn new(body: SoapDeleteUserResponse) -> Self {
                DeleteUserResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddAccessRightRequest {
                        #[yaserde(rename = "addAccessRight", default)]
                        pub body: ports::AddAccessRightRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddAccessRightRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddAccessRightRequest,
        }
        
        impl AddAccessRightRequestSoapEnvelope {
            pub fn new(body: SoapAddAccessRightRequest) -> Self {
                AddAccessRightRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddAccessRightResponse {
                    #[yaserde(rename = "AddAccessRightResponse", default)]
                    pub body: ports::AddAccessRightResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddAccessRightResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddAccessRightResponse,
        }
        
        impl AddAccessRightResponseSoapEnvelope {
            pub fn new(body: SoapAddAccessRightResponse) -> Self {
                AddAccessRightResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteAccessRightRequest {
                        #[yaserde(rename = "deleteAccessRight", default)]
                        pub body: ports::DeleteAccessRightRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteAccessRightRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteAccessRightRequest,
        }
        
        impl DeleteAccessRightRequestSoapEnvelope {
            pub fn new(body: SoapDeleteAccessRightRequest) -> Self {
                DeleteAccessRightRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteAccessRightResponse {
                    #[yaserde(rename = "DeleteAccessRightResponse", default)]
                    pub body: ports::DeleteAccessRightResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteAccessRightResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteAccessRightResponse,
        }
        
        impl DeleteAccessRightResponseSoapEnvelope {
            pub fn new(body: SoapDeleteAccessRightResponse) -> Self {
                DeleteAccessRightResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetPostboxJournalRequest {
                        #[yaserde(rename = "getPostboxJournal", default)]
                        pub body: ports::GetPostboxJournalRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetPostboxJournalRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetPostboxJournalRequest,
        }
        
        impl GetPostboxJournalRequestSoapEnvelope {
            pub fn new(body: SoapGetPostboxJournalRequest) -> Self {
                GetPostboxJournalRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetPostboxJournalResponse {
                    #[yaserde(rename = "GetPostboxJournalResponse", default)]
                    pub body: ports::GetPostboxJournalResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetPostboxJournalResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetPostboxJournalResponse,
        }
        
        impl GetPostboxJournalResponseSoapEnvelope {
            pub fn new(body: SoapGetPostboxJournalResponse) -> Self {
                GetPostboxJournalResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageJournalRequest {
                        #[yaserde(rename = "getMessageJournal", default)]
                        pub body: ports::GetMessageJournalRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageJournalRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageJournalRequest,
        }
        
        impl GetMessageJournalRequestSoapEnvelope {
            pub fn new(body: SoapGetMessageJournalRequest) -> Self {
                GetMessageJournalRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageJournalResponse {
                    #[yaserde(rename = "GetMessageJournalResponse", default)]
                    pub body: ports::GetMessageJournalResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageJournalResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageJournalResponse,
        }
        
        impl GetMessageJournalResponseSoapEnvelope {
            pub fn new(body: SoapGetMessageJournalResponse) -> Self {
                GetMessageJournalResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetAddressbookRequest {
                        #[yaserde(rename = "getAddressbook", default)]
                        pub body: ports::GetAddressbookRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetAddressbookRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetAddressbookRequest,
        }
        
        impl GetAddressbookRequestSoapEnvelope {
            pub fn new(body: SoapGetAddressbookRequest) -> Self {
                GetAddressbookRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetAddressbookResponse {
                    #[yaserde(rename = "GetAddressbookResponse", default)]
                    pub body: ports::GetAddressbookResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetAddressbookResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetAddressbookResponse,
        }
        
        impl GetAddressbookResponseSoapEnvelope {
            pub fn new(body: SoapGetAddressbookResponse) -> Self {
                GetAddressbookResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteAddressbookEntryRequest {
                        #[yaserde(rename = "deleteAddressbookEntry", default)]
                        pub body: ports::DeleteAddressbookEntryRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteAddressbookEntryRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteAddressbookEntryRequest,
        }
        
        impl DeleteAddressbookEntryRequestSoapEnvelope {
            pub fn new(body: SoapDeleteAddressbookEntryRequest) -> Self {
                DeleteAddressbookEntryRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapDeleteAddressbookEntryResponse {
                    #[yaserde(rename = "DeleteAddressbookEntryResponse", default)]
                    pub body: ports::DeleteAddressbookEntryResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct DeleteAddressbookEntryResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapDeleteAddressbookEntryResponse,
        }
        
        impl DeleteAddressbookEntryResponseSoapEnvelope {
            pub fn new(body: SoapDeleteAddressbookEntryResponse) -> Self {
                DeleteAddressbookEntryResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddAddressbookEntryRequest {
                        #[yaserde(rename = "addAddressbookEntry", default)]
                        pub body: ports::AddAddressbookEntryRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddAddressbookEntryRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddAddressbookEntryRequest,
        }
        
        impl AddAddressbookEntryRequestSoapEnvelope {
            pub fn new(body: SoapAddAddressbookEntryRequest) -> Self {
                AddAddressbookEntryRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapAddAddressbookEntryResponse {
                    #[yaserde(rename = "AddAddressbookEntryResponse", default)]
                    pub body: ports::AddAddressbookEntryResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct AddAddressbookEntryResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapAddAddressbookEntryResponse,
        }
        
        impl AddAddressbookEntryResponseSoapEnvelope {
            pub fn new(body: SoapAddAddressbookEntryResponse) -> Self {
                AddAddressbookEntryResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetVerificationResultRequest {
                        #[yaserde(rename = "getVerificationResult", default)]
                        pub body: ports::GetVerificationResultRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetVerificationResultRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetVerificationResultRequest,
        }
        
        impl GetVerificationResultRequestSoapEnvelope {
            pub fn new(body: SoapGetVerificationResultRequest) -> Self {
                GetVerificationResultRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetVerificationResultResponse {
                    #[yaserde(rename = "GetVerificationResultResponse", default)]
                    pub body: ports::GetVerificationResultResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetVerificationResultResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetVerificationResultResponse,
        }
        
        impl GetVerificationResultResponseSoapEnvelope {
            pub fn new(body: SoapGetVerificationResultResponse) -> Self {
                GetVerificationResultResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetVerificationConfigRequest {
                        #[yaserde(rename = "getVerificationConfig", default)]
                        pub body: ports::GetVerificationConfigRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetVerificationConfigRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetVerificationConfigRequest,
        }
        
        impl GetVerificationConfigRequestSoapEnvelope {
            pub fn new(body: SoapGetVerificationConfigRequest) -> Self {
                GetVerificationConfigRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetVerificationConfigResponse {
                    #[yaserde(rename = "GetVerificationConfigResponse", default)]
                    pub body: ports::GetVerificationConfigResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetVerificationConfigResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetVerificationConfigResponse,
        }
        
        impl GetVerificationConfigResponseSoapEnvelope {
            pub fn new(body: SoapGetVerificationConfigResponse) -> Self {
                GetVerificationConfigResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageConfigRequest {
                        #[yaserde(rename = "getMessageConfig", default)]
                        pub body: ports::GetMessageConfigRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageConfigRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageConfigRequest,
        }
        
        impl GetMessageConfigRequestSoapEnvelope {
            pub fn new(body: SoapGetMessageConfigRequest) -> Self {
                GetMessageConfigRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageConfigResponse {
                    #[yaserde(rename = "GetMessageConfigResponse", default)]
                    pub body: ports::GetMessageConfigResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageConfigResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageConfigResponse,
        }
        
        impl GetMessageConfigResponseSoapEnvelope {
            pub fn new(body: SoapGetMessageConfigResponse) -> Self {
                GetMessageConfigResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetConfigurationRequest {
                        #[yaserde(rename = "getConfiguration", default)]
                        pub body: ports::GetConfigurationRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetConfigurationRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetConfigurationRequest,
        }
        
        impl GetConfigurationRequestSoapEnvelope {
            pub fn new(body: SoapGetConfigurationRequest) -> Self {
                GetConfigurationRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetConfigurationResponse {
                    #[yaserde(rename = "GetConfigurationResponse", default)]
                    pub body: ports::GetConfigurationResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetConfigurationResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetConfigurationResponse,
        }
        
        impl GetConfigurationResponseSoapEnvelope {
            pub fn new(body: SoapGetConfigurationResponse) -> Self {
                GetConfigurationResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetRecipientDataRequest {
                        #[yaserde(rename = "getRecipientData", default)]
                        pub body: ports::GetRecipientDataRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetRecipientDataRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetRecipientDataRequest,
        }
        
        impl GetRecipientDataRequestSoapEnvelope {
            pub fn new(body: SoapGetRecipientDataRequest) -> Self {
                GetRecipientDataRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetRecipientDataResponse {
                    #[yaserde(rename = "GetRecipientDataResponse", default)]
                    pub body: ports::GetRecipientDataResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetRecipientDataResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetRecipientDataResponse,
        }
        
        impl GetRecipientDataResponseSoapEnvelope {
            pub fn new(body: SoapGetRecipientDataResponse) -> Self {
                GetRecipientDataResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetIdentityDataRequest {
                        #[yaserde(rename = "getIdentityData", default)]
                        pub body: ports::GetIdentityDataRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetIdentityDataRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetIdentityDataRequest,
        }
        
        impl GetIdentityDataRequestSoapEnvelope {
            pub fn new(body: SoapGetIdentityDataRequest) -> Self {
                GetIdentityDataRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetIdentityDataResponse {
                    #[yaserde(rename = "GetIdentityDataResponse", default)]
                    pub body: ports::GetIdentityDataResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetIdentityDataResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetIdentityDataResponse,
        }
        
        impl GetIdentityDataResponseSoapEnvelope {
            pub fn new(body: SoapGetIdentityDataResponse) -> Self {
                GetIdentityDataResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetPostboxesRequest {
                        #[yaserde(rename = "getPostboxes", default)]
                        pub body: ports::GetPostboxesRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetPostboxesRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetPostboxesRequest,
        }
        
        impl GetPostboxesRequestSoapEnvelope {
            pub fn new(body: SoapGetPostboxesRequest) -> Self {
                GetPostboxesRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetPostboxesResponse {
                    #[yaserde(rename = "GetPostboxesResponse", default)]
                    pub body: ports::GetPostboxesResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetPostboxesResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetPostboxesResponse,
        }
        
        impl GetPostboxesResponseSoapEnvelope {
            pub fn new(body: SoapGetPostboxesResponse) -> Self {
                GetPostboxesResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetProcesscardsRequest {
                        #[yaserde(rename = "getProcesscards", default)]
                        pub body: ports::GetProcesscardsRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetProcesscardsRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetProcesscardsRequest,
        }
        
        impl GetProcesscardsRequestSoapEnvelope {
            pub fn new(body: SoapGetProcesscardsRequest) -> Self {
                GetProcesscardsRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetProcesscardsResponse {
                    #[yaserde(rename = "GetProcesscardsResponse", default)]
                    pub body: ports::GetProcesscardsResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetProcesscardsResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetProcesscardsResponse,
        }
        
        impl GetProcesscardsResponseSoapEnvelope {
            pub fn new(body: SoapGetProcesscardsResponse) -> Self {
                GetProcesscardsResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSetProcesscardsRequest {
                        #[yaserde(rename = "setProcesscards", default)]
                        pub body: ports::SetProcesscardsRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SetProcesscardsRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSetProcesscardsRequest,
        }
        
        impl SetProcesscardsRequestSoapEnvelope {
            pub fn new(body: SoapSetProcesscardsRequest) -> Self {
                SetProcesscardsRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapSetProcesscardsResponse {
                    #[yaserde(rename = "SetProcesscardsResponse", default)]
                    pub body: ports::SetProcesscardsResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct SetProcesscardsResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapSetProcesscardsResponse,
        }
        
        impl SetProcesscardsResponseSoapEnvelope {
            pub fn new(body: SoapSetProcesscardsResponse) -> Self {
                SetProcesscardsResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetAccessRightsRequest {
                        #[yaserde(rename = "getAccessRights", default)]
                        pub body: ports::GetAccessRightsRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetAccessRightsRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetAccessRightsRequest,
        }
        
        impl GetAccessRightsRequestSoapEnvelope {
            pub fn new(body: SoapGetAccessRightsRequest) -> Self {
                GetAccessRightsRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetAccessRightsResponse {
                    #[yaserde(rename = "GetAccessRightsResponse", default)]
                    pub body: ports::GetAccessRightsResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetAccessRightsResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetAccessRightsResponse,
        }
        
        impl GetAccessRightsResponseSoapEnvelope {
            pub fn new(body: SoapGetAccessRightsResponse) -> Self {
                GetAccessRightsResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetAccessRightsGrantedRequest {
                        #[yaserde(rename = "getAccessRightsGranted", default)]
                        pub body: ports::GetAccessRightsGrantedRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetAccessRightsGrantedRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetAccessRightsGrantedRequest,
        }
        
        impl GetAccessRightsGrantedRequestSoapEnvelope {
            pub fn new(body: SoapGetAccessRightsGrantedRequest) -> Self {
                GetAccessRightsGrantedRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetAccessRightsGrantedResponse {
                    #[yaserde(rename = "GetAccessRightsGrantedResponse", default)]
                    pub body: ports::GetAccessRightsGrantedResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetAccessRightsGrantedResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetAccessRightsGrantedResponse,
        }
        
        impl GetAccessRightsGrantedResponseSoapEnvelope {
            pub fn new(body: SoapGetAccessRightsGrantedResponse) -> Self {
                GetAccessRightsGrantedResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageOverviewRequest {
                        #[yaserde(rename = "getMessageOverview", default)]
                        pub body: ports::GetMessageOverviewRequest,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageOverviewRequestSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageOverviewRequest,
        }
        
        impl GetMessageOverviewRequestSoapEnvelope {
            pub fn new(body: SoapGetMessageOverviewRequest) -> Self {
                GetMessageOverviewRequestSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapGetMessageOverviewResponse {
                    #[yaserde(rename = "GetMessageOverviewResponse", default)]
                    pub body: ports::GetMessageOverviewResponse,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<ports::SoapBeaFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct GetMessageOverviewResponseSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapGetMessageOverviewResponse,
        }
        
        impl GetMessageOverviewResponseSoapEnvelope {
            pub fn new(body: SoapGetMessageOverviewResponse) -> Self {
                GetMessageOverviewResponseSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                impl Default for BeAServiceV5HttpBinding {
                fn default() -> Self {
                    BeAServiceV5HttpBinding {
                        client: reqwest::Client::new(),
                        url: "http://brak.de/bea/application/dto/soap/types5".to_string(),
                        credentials: Option::None,
                     }
                }
            }
            impl BeAServiceV5HttpBinding {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
                    BeAServiceV5HttpBinding {
                        client: reqwest::Client::new(),
                        url: url.to_string(),
                        credentials,
                    }
                }
        }
        pub struct BeAServiceV5HttpBinding {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                #[async_trait]
	impl ports::BeAServiceV5PortType for BeAServiceV5HttpBinding {
	async fn get_message (&self, get_message_request: ports::GetMessageRequest) -> Result<ports::GetMessageResponse, Option<ports::SoapBeaFault>> {

        let __request = GetMessageRequestSoapEnvelope::new(SoapGetMessageRequest {
            body: get_message_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getMessage")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetMessageResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn create_new_message (&self, create_new_message_request: ports::CreateNewMessageRequest) -> Result<ports::CreateNewMessageResponse, Option<ports::SoapBeaFault>> {

        let __request = CreateNewMessageRequestSoapEnvelope::new(SoapCreateNewMessageRequest {
            body: create_new_message_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/createNewMessage")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: CreateNewMessageResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn save_message (&self, save_message_request: ports::SaveMessageRequest) -> Result<ports::SaveMessageResponse, Option<ports::SoapBeaFault>> {

        let __request = SaveMessageRequestSoapEnvelope::new(SoapSaveMessageRequest {
            body: save_message_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/saveMessage")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: SaveMessageResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn send_message (&self, send_message_request: ports::SendMessageRequest) -> Result<ports::SendMessageResponse, Option<ports::SoapBeaFault>> {

        let __request = SendMessageRequestSoapEnvelope::new(SoapSendMessageRequest {
            body: send_message_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/sendMessage")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: SendMessageResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_folder_structure (&self, get_folder_structure_request: ports::GetFolderStructureRequest) -> Result<ports::GetFolderStructureResponse, Option<ports::SoapBeaFault>> {

        let __request = GetFolderStructureRequestSoapEnvelope::new(SoapGetFolderStructureRequest {
            body: get_folder_structure_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getFolderStructure")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetFolderStructureResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_folder_overview (&self, get_folder_overview_request: ports::GetFolderOverviewRequest) -> Result<ports::GetFolderOverviewResponse, Option<ports::SoapBeaFault>> {

        let __request = GetFolderOverviewRequestSoapEnvelope::new(SoapGetFolderOverviewRequest {
            body: get_folder_overview_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getFolderOverview")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetFolderOverviewResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn search_addressee (&self, search_addressee_request: ports::SearchAddresseeRequest) -> Result<ports::SearchAddresseeResponse, Option<ports::SoapBeaFault>> {

        let __request = SearchAddresseeRequestSoapEnvelope::new(SoapSearchAddresseeRequest {
            body: search_addressee_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/searchAddressee")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: SearchAddresseeResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_favourite_addressee (&self, get_favourite_addressee_request: ports::GetFavouriteAddresseeRequest) -> Result<ports::GetFavouriteAddresseeResponse, Option<ports::SoapBeaFault>> {

        let __request = GetFavouriteAddresseeRequestSoapEnvelope::new(SoapGetFavouriteAddresseeRequest {
            body: get_favourite_addressee_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getFavouriteAddressee")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetFavouriteAddresseeResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn delete_favourite_addressee (&self, delete_favourite_addressee_request: ports::DeleteFavouriteAddresseeRequest) -> Result<ports::DeleteFavouriteAddresseeResponse, Option<ports::SoapBeaFault>> {

        let __request = DeleteFavouriteAddresseeRequestSoapEnvelope::new(SoapDeleteFavouriteAddresseeRequest {
            body: delete_favourite_addressee_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/deleteFavouriteAddressee")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: DeleteFavouriteAddresseeResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn move_message_to_folder (&self, move_message_to_folder_request: ports::MoveMessageToFolderRequest) -> Result<ports::MoveMessageToFolderResponse, Option<ports::SoapBeaFault>> {

        let __request = MoveMessageToFolderRequestSoapEnvelope::new(SoapMoveMessageToFolderRequest {
            body: move_message_to_folder_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/moveMessageToFolder")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: MoveMessageToFolderResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn move_message_to_trash (&self, move_message_to_trash_request: ports::MoveMessageToTrashRequest) -> Result<ports::MoveMessageToTrashResponse, Option<ports::SoapBeaFault>> {

        let __request = MoveMessageToTrashRequestSoapEnvelope::new(SoapMoveMessageToTrashRequest {
            body: move_message_to_trash_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/moveMessageToTrash")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: MoveMessageToTrashResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn restore_message_from_trash (&self, restore_message_from_trash_request: ports::RestoreMessageFromTrashRequest) -> Result<ports::RestoreMessageFromTrashResponse, Option<ports::SoapBeaFault>> {

        let __request = RestoreMessageFromTrashRequestSoapEnvelope::new(SoapRestoreMessageFromTrashRequest {
            body: restore_message_from_trash_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/restoreMessageFromTrash")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: RestoreMessageFromTrashResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn delete_message (&self, delete_message_request: ports::DeleteMessageRequest) -> Result<ports::DeleteMessageResponse, Option<ports::SoapBeaFault>> {

        let __request = DeleteMessageRequestSoapEnvelope::new(SoapDeleteMessageRequest {
            body: delete_message_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/deleteMessage")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: DeleteMessageResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn add_folder (&self, add_folder_request: ports::AddFolderRequest) -> Result<ports::AddFolderResponse, Option<ports::SoapBeaFault>> {

        let __request = AddFolderRequestSoapEnvelope::new(SoapAddFolderRequest {
            body: add_folder_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/addFolder")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: AddFolderResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn update_folder (&self, update_folder_request: ports::UpdateFolderRequest) -> Result<ports::UpdateFolderResponse, Option<ports::SoapBeaFault>> {

        let __request = UpdateFolderRequestSoapEnvelope::new(SoapUpdateFolderRequest {
            body: update_folder_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/updateFolder")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: UpdateFolderResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn remove_folder (&self, remove_folder_request: ports::RemoveFolderRequest) -> Result<ports::RemoveFolderResponse, Option<ports::SoapBeaFault>> {

        let __request = RemoveFolderRequestSoapEnvelope::new(SoapRemoveFolderRequest {
            body: remove_folder_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/removeFolder")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: RemoveFolderResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn add_comment (&self, add_comment_request: ports::AddCommentRequest) -> Result<ports::AddCommentResponse, Option<ports::SoapBeaFault>> {

        let __request = AddCommentRequestSoapEnvelope::new(SoapAddCommentRequest {
            body: add_comment_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/addComment")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: AddCommentResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn update_comment (&self, update_comment_request: ports::UpdateCommentRequest) -> Result<ports::UpdateCommentResponse, Option<ports::SoapBeaFault>> {

        let __request = UpdateCommentRequestSoapEnvelope::new(SoapUpdateCommentRequest {
            body: update_comment_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/updateComment")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: UpdateCommentResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn remove_comment (&self, remove_comment_request: ports::RemoveCommentRequest) -> Result<ports::RemoveCommentResponse, Option<ports::SoapBeaFault>> {

        let __request = RemoveCommentRequestSoapEnvelope::new(SoapRemoveCommentRequest {
            body: remove_comment_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/removeComment")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: RemoveCommentResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_user_name (&self, get_user_name_request: ports::GetUserNameRequest) -> Result<ports::GetUserNameResponse, Option<ports::SoapBeaFault>> {

        let __request = GetUserNameRequestSoapEnvelope::new(SoapGetUserNameRequest {
            body: get_user_name_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getUserName")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetUserNameResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_user_journal (&self, get_user_journal_request: ports::GetUserJournalRequest) -> Result<ports::GetUserJournalResponse, Option<ports::SoapBeaFault>> {

        let __request = GetUserJournalRequestSoapEnvelope::new(SoapGetUserJournalRequest {
            body: get_user_journal_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getUserJournal")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetUserJournalResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn delete_authentification (&self, delete_authentification_request: ports::DeleteAuthentificationRequest) -> Result<ports::DeleteAuthentificationResponse, Option<ports::SoapBeaFault>> {

        let __request = DeleteAuthentificationRequestSoapEnvelope::new(SoapDeleteAuthentificationRequest {
            body: delete_authentification_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/deleteAuthentification")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: DeleteAuthentificationResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn add_authentification (&self, add_authentification_request: ports::AddAuthentificationRequest) -> Result<ports::AddAuthentificationResponse, Option<ports::SoapBeaFault>> {

        let __request = AddAuthentificationRequestSoapEnvelope::new(SoapAddAuthentificationRequest {
            body: add_authentification_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/addAuthentification")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: AddAuthentificationResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn start_registration (&self, start_registration_request: ports::StartRegistrationRequest) -> Result<ports::StartRegistrationResponse, Option<ports::SoapBeaFault>> {

        let __request = StartRegistrationRequestSoapEnvelope::new(SoapStartRegistrationRequest {
            body: start_registration_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/startRegistration")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: StartRegistrationResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn finish_registration (&self, finish_registration_request: ports::FinishRegistrationRequest) -> Result<ports::FinishRegistrationResponse, Option<ports::SoapBeaFault>> {

        let __request = FinishRegistrationRequestSoapEnvelope::new(SoapFinishRegistrationRequest {
            body: finish_registration_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/finishRegistration")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FinishRegistrationResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn login_user (&self, login_user_request: ports::LoginUserRequest) -> Result<ports::LoginUserResponse, Option<ports::SoapBeaFault>> {

        let __request = LoginUserRequestSoapEnvelope::new(SoapLoginUserRequest {
            body: login_user_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/loginUser")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: LoginUserResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn logout_user (&self, logout_user_request: ports::LogoutUserRequest) -> Result<ports::LogoutUserResponse, Option<ports::SoapBeaFault>> {

        let __request = LogoutUserRequestSoapEnvelope::new(SoapLogoutUserRequest {
            body: logout_user_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/logoutUser")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: LogoutUserResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn start_login_user (&self, start_login_user_request: ports::StartLoginUserRequest) -> Result<ports::StartLoginUserResponse, Option<ports::SoapBeaFault>> {

        let __request = StartLoginUserRequestSoapEnvelope::new(SoapStartLoginUserRequest {
            body: start_login_user_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/startLoginUser")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: StartLoginUserResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn add_user (&self, add_user_request: ports::AddUserRequest) -> Result<ports::AddUserResponse, Option<ports::SoapBeaFault>> {

        let __request = AddUserRequestSoapEnvelope::new(SoapAddUserRequest {
            body: add_user_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/addUser")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: AddUserResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn update_user (&self, update_user_request: ports::UpdateUserRequest) -> Result<ports::UpdateUserResponse, Option<ports::SoapBeaFault>> {

        let __request = UpdateUserRequestSoapEnvelope::new(SoapUpdateUserRequest {
            body: update_user_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/updateUser")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: UpdateUserResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn delete_user (&self, delete_user_request: ports::DeleteUserRequest) -> Result<ports::DeleteUserResponse, Option<ports::SoapBeaFault>> {

        let __request = DeleteUserRequestSoapEnvelope::new(SoapDeleteUserRequest {
            body: delete_user_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/deleteUser")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: DeleteUserResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn add_access_right (&self, add_access_right_request: ports::AddAccessRightRequest) -> Result<ports::AddAccessRightResponse, Option<ports::SoapBeaFault>> {

        let __request = AddAccessRightRequestSoapEnvelope::new(SoapAddAccessRightRequest {
            body: add_access_right_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/addAccessRight")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: AddAccessRightResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn delete_access_right (&self, delete_access_right_request: ports::DeleteAccessRightRequest) -> Result<ports::DeleteAccessRightResponse, Option<ports::SoapBeaFault>> {

        let __request = DeleteAccessRightRequestSoapEnvelope::new(SoapDeleteAccessRightRequest {
            body: delete_access_right_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/deleteAccessRight")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: DeleteAccessRightResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_postbox_journal (&self, get_postbox_journal_request: ports::GetPostboxJournalRequest) -> Result<ports::GetPostboxJournalResponse, Option<ports::SoapBeaFault>> {

        let __request = GetPostboxJournalRequestSoapEnvelope::new(SoapGetPostboxJournalRequest {
            body: get_postbox_journal_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getPostboxJournal")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetPostboxJournalResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_message_journal (&self, get_message_journal_request: ports::GetMessageJournalRequest) -> Result<ports::GetMessageJournalResponse, Option<ports::SoapBeaFault>> {

        let __request = GetMessageJournalRequestSoapEnvelope::new(SoapGetMessageJournalRequest {
            body: get_message_journal_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getMessageJournal")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetMessageJournalResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_addressbook (&self, get_addressbook_request: ports::GetAddressbookRequest) -> Result<ports::GetAddressbookResponse, Option<ports::SoapBeaFault>> {

        let __request = GetAddressbookRequestSoapEnvelope::new(SoapGetAddressbookRequest {
            body: get_addressbook_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getAddressbook")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetAddressbookResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn delete_addressbook_entry (&self, delete_addressbook_entry_request: ports::DeleteAddressbookEntryRequest) -> Result<ports::DeleteAddressbookEntryResponse, Option<ports::SoapBeaFault>> {

        let __request = DeleteAddressbookEntryRequestSoapEnvelope::new(SoapDeleteAddressbookEntryRequest {
            body: delete_addressbook_entry_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/deleteAddressbookEntry")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: DeleteAddressbookEntryResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn add_addressbook_entry (&self, add_addressbook_entry_request: ports::AddAddressbookEntryRequest) -> Result<ports::AddAddressbookEntryResponse, Option<ports::SoapBeaFault>> {

        let __request = AddAddressbookEntryRequestSoapEnvelope::new(SoapAddAddressbookEntryRequest {
            body: add_addressbook_entry_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/addAddressbookEntry")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: AddAddressbookEntryResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_verification_result (&self, get_verification_result_request: ports::GetVerificationResultRequest) -> Result<ports::GetVerificationResultResponse, Option<ports::SoapBeaFault>> {

        let __request = GetVerificationResultRequestSoapEnvelope::new(SoapGetVerificationResultRequest {
            body: get_verification_result_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getVerificationResult")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetVerificationResultResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_verification_config (&self, get_verification_config_request: ports::GetVerificationConfigRequest) -> Result<ports::GetVerificationConfigResponse, Option<ports::SoapBeaFault>> {

        let __request = GetVerificationConfigRequestSoapEnvelope::new(SoapGetVerificationConfigRequest {
            body: get_verification_config_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getVerificationConfig")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetVerificationConfigResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_message_config (&self, get_message_config_request: ports::GetMessageConfigRequest) -> Result<ports::GetMessageConfigResponse, Option<ports::SoapBeaFault>> {

        let __request = GetMessageConfigRequestSoapEnvelope::new(SoapGetMessageConfigRequest {
            body: get_message_config_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getMessageConfig")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetMessageConfigResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_configuration (&self, get_configuration_request: ports::GetConfigurationRequest) -> Result<ports::GetConfigurationResponse, Option<ports::SoapBeaFault>> {

        let __request = GetConfigurationRequestSoapEnvelope::new(SoapGetConfigurationRequest {
            body: get_configuration_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getConfiguration")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetConfigurationResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_recipient_data (&self, get_recipient_data_request: ports::GetRecipientDataRequest) -> Result<ports::GetRecipientDataResponse, Option<ports::SoapBeaFault>> {

        let __request = GetRecipientDataRequestSoapEnvelope::new(SoapGetRecipientDataRequest {
            body: get_recipient_data_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getRecipientData")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetRecipientDataResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_identity_data (&self, get_identity_data_request: ports::GetIdentityDataRequest) -> Result<ports::GetIdentityDataResponse, Option<ports::SoapBeaFault>> {

        let __request = GetIdentityDataRequestSoapEnvelope::new(SoapGetIdentityDataRequest {
            body: get_identity_data_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getIdentityData")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetIdentityDataResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_postboxes (&self, get_postboxes_request: ports::GetPostboxesRequest) -> Result<ports::GetPostboxesResponse, Option<ports::SoapBeaFault>> {

        let __request = GetPostboxesRequestSoapEnvelope::new(SoapGetPostboxesRequest {
            body: get_postboxes_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getPostboxes")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetPostboxesResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_processcards (&self, get_processcards_request: ports::GetProcesscardsRequest) -> Result<ports::GetProcesscardsResponse, Option<ports::SoapBeaFault>> {

        let __request = GetProcesscardsRequestSoapEnvelope::new(SoapGetProcesscardsRequest {
            body: get_processcards_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getProcesscards")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetProcesscardsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn set_processcards (&self, set_processcards_request: ports::SetProcesscardsRequest) -> Result<ports::SetProcesscardsResponse, Option<ports::SoapBeaFault>> {

        let __request = SetProcesscardsRequestSoapEnvelope::new(SoapSetProcesscardsRequest {
            body: set_processcards_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/setProcesscards")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: SetProcesscardsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_access_rights (&self, get_access_rights_request: ports::GetAccessRightsRequest) -> Result<ports::GetAccessRightsResponse, Option<ports::SoapBeaFault>> {

        let __request = GetAccessRightsRequestSoapEnvelope::new(SoapGetAccessRightsRequest {
            body: get_access_rights_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getAccessRights")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetAccessRightsResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_access_rights_granted (&self, get_access_rights_granted_request: ports::GetAccessRightsGrantedRequest) -> Result<ports::GetAccessRightsGrantedResponse, Option<ports::SoapBeaFault>> {

        let __request = GetAccessRightsGrantedRequestSoapEnvelope::new(SoapGetAccessRightsGrantedRequest {
            body: get_access_rights_granted_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getAccessRightsGranted")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetAccessRightsGrantedResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn get_message_overview (&self, get_message_overview_request: ports::GetMessageOverviewRequest) -> Result<ports::GetMessageOverviewResponse, Option<ports::SoapBeaFault>> {

        let __request = GetMessageOverviewRequestSoapEnvelope::new(SoapGetMessageOverviewRequest {
            body: get_message_overview_request,
            xmlns: Option::Some("http://brak.de/bea/application/dto/soap/types5".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://brak.bea/getMessageOverview")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: GetMessageOverviewResponseSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}}
}

pub mod services {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            pub struct BeAServiceV5 {}
               impl BeAServiceV5 {
                
            pub fn new_client(credentials: Option<(String, String)>) -> bindings::BeAServiceV5HttpBinding {
                bindings::BeAServiceV5HttpBinding::new("https://www.brak.de/bea/BeAServiceV5", credentials)
            }
        }
}

