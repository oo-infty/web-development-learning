<%@ Language="VBScript" %>
<%
  Response.ContentType = "text/xml"

  Dim xmlDocument
  Set xmlDocument = Server.CreateObject("MSXML2.DOMDocument.6.0")

  Set xmlDecl = xmlDocument.createProcessingInstruction("xml", "version='1.0' encoding='UTF-8'")
  xmlDocument.appendChild(xmlDecl)

  Set root = xmlDocument.createElement("root")
  xmlDocument.appendChild(root)

  ' Helper functions
  Function CreateSelection(questionType, num, contentStr, optionA, optionB, optionC, optionD)
    Set question = xmlDocument.createElement(questionType & "-selection")

    Set id = xmlDocument.createElement("id")
    id.text = num
    question.appendChild(id)

    Set content = xmlDocument.createElement("content")
    Set data = xmlDocument.createCDATASection(contentStr)
    content.appendChild(data)
    question.appendChild(content)

    Set content = xmlDocument.createElement("option")
    Set data = xmlDocument.createCDATASection(optionA)
    content.appendChild(data)
    question.appendChild(content)

    Set content = xmlDocument.createElement("option")
    Set data = xmlDocument.createCDATASection(optionB)
    content.appendChild(data)
    question.appendChild(content)

    Set content = xmlDocument.createElement("option")
    Set data = xmlDocument.createCDATASection(optionC)
    content.appendChild(data)
    question.appendChild(content)

    Set content = xmlDocument.createElement("option")
    Set data = xmlDocument.createCDATASection(optionD)
    content.appendChild(data)
    question.appendChild(content)

    Set CreateSelection = question
  End Function

  Function CreateSingleSelection(num, contentStr, optionA, optionB, optionC, optionD)
    Set CreateSingleSelection = CreateSelection("single", num, contentStr, optionA, optionB, optionC, optionD)
  End Function

  Function CreateMultipleSelection(num, contentStr, optionA, optionB, optionC, optionD)
    Set CreateMultipleSelection = CreateSelection("multiple", num, contentStr, optionA, optionB, optionC, optionD)
  End Function

  Function CreateCompletion(num, contentStr)
    Set question = xmlDocument.createElement("completion")
    Set id = xmlDocument.createElement("id")
    id.text = num
    question.appendChild(id)

    Set content = xmlDocument.createElement("content")
    Set data = xmlDocument.createCDATASection(contentStr)
    content.appendChild(data)
    question.appendChild(content)

    Set CreateCompletion = question
  End Function

  ' Construct XML
  root.appendChild( _
    CreateSingleSelection( _
      1, _
      "Which command is used to trace the system calls made by a process, and which options would you use to trace a specific process ID (PID) and output the results to a file?", _
      "<code>strace -p PID -o output.txt</code>", _
      "<code>strace -c -p PID > output.txt</code>", _
      "<code>strace -f -p PID | tee output.txt</code>", _
      "<code>strace -t -p PID > output.txt</code>" _
    ) _
  )

  root.appendChild( _
    CreateMultipleSelection( _
      2, _
      "In Linux, how can you check the IP address of network interfaces?", _
      "<code>ifconfig</code>", _
      "<code>ip addr show</code>", _
      "<code>netstat</code>", _
      "<code>ping</code>" _
    ) _
  )

  root.appendChild( _
    CreateCompletion( _
      3, _
      "In Linux, which commands can be used to find files or directories?" _
    ) _
  )

  Response.Write(xmlDocument.xml)
%>
