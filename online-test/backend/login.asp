<%@ Language="VBScript" %>
<%
  Dim username
  Set username = Request.Form("username")
  Session("username") = username
  Session.Timeout = 45
%>
