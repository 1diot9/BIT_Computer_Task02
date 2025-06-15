import copy
import io
import os

from flask import Flask, send_from_directory, make_response
from flask import render_template
from flask import request

from qshifter import QuickShifter, QuickShifterLines

app = Flask(__name__)


# index页面
@app.route("/")
def hello_world(name="world"):  # put application's code here
    if request.args.get("name") is not None:
        name = request.args.get("name")
    return render_template("test.html", name=name)


# 文本输入路由
@app.route("/qshifter")
def qshifter():
    return render_template("form.html")


@app.route("/api/qshifter", methods=["POST"])
def api_qshifter():

    # 文本输入处理
    res = request.form.get("qshift")
    if res:
        test = QuickShifter(res)
        array = test.shifts
        # 将结果传递到视图模板中，遍历输出其中的元素
        return render_template("result.html", result=array)

    # TODO: 增加文件上传防护
    # 文件上传处理
    array = []
    file = request.files.get("file")
    output = file
    if file:
        filename = file.filename
        # 将上传的文件留档
        # file.save(f'uploads/{filename}')
        # 读取文件内容并逐行处理
        content = file.stream.read().decode("utf-8").splitlines()
        # 在这里可以对每一行进行处理
        processed_lines = [line.strip() for line in content]
        count = processed_lines.__len__()
        result = []

        # 所有句子一起排序
        text = QuickShifterLines(processed_lines, merge=True)
        result = text.lshifts

        # 每个句子单独排序
        """
        for line in processed_lines:
            text = QuickShifter(line)
            array = text.shifts
            result.append(array)
        """

        # 增加result下载功能
        with open("./tmp/output.txt", "w") as f:
            for i in result:
                for j in i:
                    f.write(j + "\n")
            output = f

        return render_template(
            "result_file.html", result=result, count=count, output=output
        )
    else:
        return "请输入文本或选择文件！！！"


# 文件下载
@app.route("/download")
def download_file():

    with open("./tmp/output.txt", "rb") as f:
        output = f.read()

    mem_io = io.BytesIO()
    mem_io.write(output)
    mem_io.seek(0)

    response = make_response(mem_io.getvalue())
    response.headers["Content-Type"] = "application/octet-stream"
    response.headers["Content-Disposition"] = "attachment; filename=result.txt"
    return response

@app.route("/api/search", methods=["POST"])
def search_keywords():
    keywords = request.form.get("keywords")
    if keywords == "":
        return "无输入"
    keywords_list = keywords.split(" ")
    with open("./tmp/output.txt", "r") as f:
        splited = []
        while True:
            line = f.readline()
            if line == "":
                break
            re = line.strip().split()
            splited.append(re)
        print(splited)

        no = 0
        no_list = []
        for inner in splited:
            keywords = copy.deepcopy(keywords_list)
            for word in inner:
                for key in keywords:
                    if key == word:
                        keywords.remove(key)
                        break
                if keywords == []:
                    print(no)
                    no_list.append(no)
                    break
            no += 1
    return render_template("search_result.html", no_list=no_list, keywords=keywords_list)


if __name__ == "__main__":
    app.run()
