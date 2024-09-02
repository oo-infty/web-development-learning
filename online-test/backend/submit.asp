<%
  Dim xmlDocument
  Set xmlDocument = Server.CreateObject("MSXML2.DOMDocument.6.0")
  Set xmlDecl = xmlDocument.createProcessingInstruction("xml", "version='1.0' encoding='UTF-8'")
  xmlDocument.appendChild(xmlDecl)
  Set root = xmlDocument.createElement("root")
  xmlDocument.appendChild(root)

  Function CalcScore()
    CalcScore = 100
  End Function

  If IsEmpty(Session("username")) Then
    Set result = xmlDocument.createElement("result")
    result.text = "not-logined"
    root.appendChild(result)
  Else
    testId = Request.Form("test-id")

    If IsEmpty(Session(testId)) Then
      Set result = xmlDocument.createElement("result")
      result.text = "test-expired"
      root.appendChild(result)
    Else
      Session.Contents.Remove(testId)
      Set result = xmlDocument.createElement("result")
      result.text = "ok"
      root.appendChild(result)

      score = CalcScore()
    End if
  End if

  Response.Write(xmlDocument.xml)
%>
